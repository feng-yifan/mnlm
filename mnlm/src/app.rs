mod config;
mod socket_manager;

pub use config::*;

pub struct App {
    config: Config,
}

impl App {
    /// 创建 App 实例
    pub fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }

    /// 启动应用
    ///
    /// 创建 Unix Domain Socket 监听器, 进入事件循环。
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _manager = socket_manager::SocketManager::new();
        todo!("实现事件循环")
    }
}
