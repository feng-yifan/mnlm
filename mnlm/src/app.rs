use crate::error::Result;
use crate::gateway::Gateway;

mod config;

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
    pub async fn run(&self) -> Result<()> {
        let gateway = Gateway::new().await?;
        gateway.run().await
    }
}