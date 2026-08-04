use config::Config as ConfigBuilder;
use serde::Deserialize;

/// 应用配置
///
/// 从环境变量加载, 环境变量必须使用 MNLM_ 前缀。
/// 未来可扩展为从配置文件加载。
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub default_model: Option<String>,
}

impl Config {
    /// 从环境变量加载配置
    ///
    /// 环境变量名使用大写 + 下划线格式, 去掉 MNLM_ 前缀后映射到结构体字段。
    /// 例如 MNLM_SOCKET_ADDR 映射到 socket_addr 字段。
    ///
    /// 如果配置加载失败, 直接 panic。未来扩展文件加载时, 此处应从文件 + 环境变量合并构建。
    pub fn load() -> Self {
        ConfigBuilder::builder()
            .add_source(
                config::Environment::with_prefix("MNLM")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()
            .expect("配置加载失败, 请检查环境变量设置")
            .try_deserialize()
            .expect("配置反序列化失败, 请检查环境变量值格式")
    }
}