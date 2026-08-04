use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// I/O 错误
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),
}

/// 应用 Result 类型
pub type Result<T> = std::result::Result<T, Error>;