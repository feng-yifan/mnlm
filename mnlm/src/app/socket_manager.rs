use std::os::unix::net::UnixListener;
use std::path::PathBuf;

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

    /// 获取套接字文件路径
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// 创建 Unix Domain Socket 监听器
    ///
    /// 绑定前会清理已存在的套接字文件, 避免 `Address already in use` 错误。
    /// 绑定后设置权限为 0o600, 仅所有者可读写。
    pub fn create_listener(&self) -> std::io::Result<UnixListener> {
        // 清理已存在的套接字文件
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
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

    /// 使用临时路径创建 SocketManager, 避免并行测试互相干扰
    fn test_manager() -> SocketManager {
        let tmp_dir = std::env::temp_dir();
        let pid = std::process::id();
        // 使用线程名区分, 避免并行测试竞争
        let tid = std::thread::current().id();
        let path = tmp_dir.join(format!("mnlm_test_{:?}_{}.sock", tid, pid));
        // 先清理可能残留的文件
        if path.exists() {
            std::fs::remove_file(&path).ok();
        }
        SocketManager { path }
    }

    #[test]
    fn test_create_listener() {
        let manager = test_manager();

        let listener = manager.create_listener();
        assert!(listener.is_ok(), "监听器创建失败: {:?}", listener.err());

        assert!(manager.path.exists(), "套接字文件应存在: {:?}", manager.path);

        let stream = UnixStream::connect(&manager.path);
        assert!(stream.is_ok(), "应能连接到套接字: {:?}", stream.err());

        let path = manager.path.clone();
        drop(manager);
        assert!(!path.exists(), "Drop 后套接字文件应被删除");
    }

    #[test]
    fn test_recreate_listener() {
        let manager = test_manager();

        let listener1 = manager.create_listener();
        assert!(listener1.is_ok());

        // 第二次创建, 旧文件被自动清理后应成功
        let listener2 = manager.create_listener();
        assert!(listener2.is_ok(), "重复创建应自动清理旧文件并成功");

        drop(manager);
    }
}