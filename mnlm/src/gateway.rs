mod socket_manager;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use crate::error::Result;
use crate::gateway::socket_manager::SocketManager;
use crate::session::Session;

/// 消息网关
///
/// 负责消息收发和双工通信, 通过 Unix Domain Socket 与客户端建立连接。
pub struct Gateway {
    listener: tokio::net::UnixListener,
    _manager: SocketManager,
}

impl Gateway {
    /// 创建网关并启动 UDS 监听
    pub async fn new() -> Result<Self> {
        let manager = SocketManager::new();
        let listener = manager.create_listener().await?;

        Ok(Self {
            listener,
            _manager: manager,
        })
    }

    /// 启动网关, 进入消息处理循环
    ///
    /// 接受客户端连接, 每个连接在独立异步任务中处理。
    pub async fn run(&self) -> Result<()> {
        println!("木牛流马启动完成, 等待连接...");

        loop {
            match self.listener.accept().await {
                Ok((stream, addr)) => {
                    println!("收到连接: {:?}", addr);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream).await {
                            eprintln!("处理连接失败: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("接受连接失败: {}", e),
            }
        }
    }
}

/// 处理客户端连接
///
/// 每个连接创建一个 session, 从 stream 逐行读取消息,
/// 发送到 session 处理, 将响应写回 stream。
async fn handle_connection(stream: UnixStream) -> Result<()> {
    let session = Session::new();
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("客户端断开连接");
                break;
            }
            Ok(_) => {
                let msg = line.trim_end();
                println!("收到消息: {}", msg);

                let response = session.handle(msg);

                writer.write_all(response.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                writer.flush().await?;
            }
            Err(e) => {
                eprintln!("读取消息失败: {}", e);
                break;
            }
        }
    }

    Ok(())
}