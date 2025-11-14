//! aliyun_content_moderation
//!
//! 这是一个简易版的 Rust 客户端，用于以 HTTPS 原生方式调用阿里云内容安全（文本/图片/音频/视频）相关接口。
//! 由于阿里并未提供 Rust 版本的 SDK，在抓耳挠腮挣扎两天终于能调用成功以后，准备为使用 Rust 的后来者做点贡献。
//!
//! 使用方式：调用ModerationClient::new().check_text();
//! 由于是简化版，只提供在 2025 年 11 月 2 日 能跑通的版本，没有扩展签名方式，当前使用的签名方式：SHA1.
//! 审核返回的类型 TextCheckResponse.
//!
//! 注意，库中的签名类已经废弃。

pub mod client;
pub mod signature;
pub mod types;
pub mod error;

// re-export
pub use client::ModerationClient;

#[cfg(test)] // 只在测试构建时编译
mod tests {
    use config::Config;
    use crate::types::AppConfig;
    use super::*; // 引入上层作用域（被测函数）

    #[tokio::test] // 标记为单元测试
    async fn test_text_moderation() {

        let config = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .unwrap();
        let app_config: AppConfig = config.try_deserialize().unwrap();

        let client = ModerationClient::new(
            "https://green-cip.cn-beijing.aliyuncs.com",
            app_config.access_key_id,
            app_config.access_key_secret,
            app_config.version
        );
        let result = client.check_text("chat_detection_pro", "电信诈骗").await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test] // 标记为单元测试
    async fn test_image_moderation() {

        let config = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .unwrap();
        let app_config: AppConfig = config.try_deserialize().unwrap();

        let client = ModerationClient::new(
            "https://green-cip.cn-beijing.aliyuncs.com",
            app_config.access_key_id,
            app_config.access_key_secret,
            app_config.version
        );
        let result = client.check_image("baselineCheck",
                                        "https://www.imageoss.com/images/2025/11/14/__206c8baf7092d8f3d.png").await;

        assert_eq!(result.is_ok(), true);
    }
}