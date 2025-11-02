use config::Config;
use aliyun_content_moderation::ModerationClient;
use aliyun_content_moderation::types::AppConfig;

#[tokio::main]
async fn main() {
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
    let result = client.check_text("nickname_detection_pro", "去死吧你").await;
    println!("{:?}", result);
}