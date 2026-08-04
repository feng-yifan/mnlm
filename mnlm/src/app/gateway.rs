use std::os::unix::net::UnixListener;

use crate::app::socket_manager::SocketManager;
use crate::error::Result;

/// 消息网关
///
/// 负责消息收发和双工通信, 通过 Unix Domain Socket 与客户端建立连接。
pub struct Gateway {
    listener: UnixListener,
    _manager: SocketManager,
}

impl Gateway {
    /// 创建网关并启动 UDS 监听
    pub fn new() -> Result<Self> {
        let manager = SocketManager::new();
        let listener = manager.create_listener()?;

        Ok(Self {
            listener,
            _manager: manager,
        })
    }

    /// 启动网关, 进入消息处理循环
    ///
    /// 接受客户端连接, 后续将实现消息收发。
    pub fn run(&self) -> Result<()> {
        println!("木牛流马启动完成, 等待连接...");

        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    println!("收到连接: {:?}", addr);
                    todo!("创建 session");
                }
                Err(e) => eprintln!("接受连接失败: {}", e),
            }
        }
    }
}
