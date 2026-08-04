use std::path::PathBuf;

use tokio::net::UnixListener;

use crate::error::Result;

/// Unix Domain Socket 路径
const SOCKET_PATH: &str = "/tmp/mnlm.sock";

/// Unix Domain Socket 管理器
///
/// 负责创建和管理 UDS 监听器。
/// 通过 Drop 自动清理套接字文件。
pub struct SocketManager {
    path: PathBuf,
}

impl SocketManager {
    /// 创建 SocketManager
    ///
    /// 使用硬编码路径 `/tmp/mnlm.sock`。
    pub fn new() -> Self {
        Self {
            path: PathBuf::from(SOCKET_PATH),
        }
    }

    /// 创建异步 Unix Domain Socket 监听器
    ///
    /// 绑定前会清理已存在的套接字文件, 避免 `Address already in use` 错误。
    /// 绑定后设置权限为 0o600, 仅所有者可读写。
    pub async fn create_listener(&self) -> Result<UnixListener> {
        if self.path.exists() {
            tokio::fs::remove_file(&self.path).await?;
        }

        let listener = UnixListener::bind(&self.path)?;

        // 设置权限为 0o600, 仅所有者可读写
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&self.path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&self.path, perms)?;

        Ok(listener)
    }
}

impl Drop for SocketManager {
    /// 进程退出时自动删除套接字文件
    fn drop(&mut self) {
        if self.path.exists() {
            std::fs::remove_file(&self.path).ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixStream;

    fn test_manager() -> SocketManager {
        let tmp_dir = std::env::temp_dir();
        let pid = std::process::id();
        let tid = std::thread::current().id();
        let path = tmp_dir.join(format!("mnlm_test_{:?}_{}.sock", tid, pid));
        if path.exists() {
            std::fs::remove_file(&path).ok();
        }
        SocketManager { path }
    }

    #[tokio::test]
    async fn test_create_listener() {
        let manager = test_manager();

        let listener = manager.create_listener().await;
        assert!(listener.is_ok(), "监听器创建失败: {:?}", listener.err());

        assert!(manager.path.exists(), "套接字文件应存在: {:?}", manager.path);

        let stream = UnixStream::connect(&manager.path);
        assert!(stream.is_ok(), "应能连接到套接字: {:?}", stream.err());

        let path = manager.path.clone();
        drop(manager);
        assert!(!path.exists(), "Drop 后套接字文件应被删除");
    }

    #[tokio::test]
    async fn test_recreate_listener() {
        let manager = test_manager();

        let listener1 = manager.create_listener().await;
        assert!(listener1.is_ok());

        let listener2 = manager.create_listener().await;
        assert!(listener2.is_ok(), "重复创建应自动清理旧文件并成功");

        drop(manager);
    }
}