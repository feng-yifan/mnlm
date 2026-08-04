/// 会话
///
/// 负责处理客户端消息, 与 AI 模型交互并返回响应。
pub struct Session {}

impl Session {
    /// 创建会话实例
    pub fn new() -> Self {
        Self {}
    }

    /// 处理消息并返回响应
    pub fn handle(&self, msg: &str) -> String {
        todo!("与 AI 模型交互: {}", msg)
    }
}