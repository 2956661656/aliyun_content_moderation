use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub version: String,
}

// 通用请求元信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonRequestMeta<'a> {
    pub region: &'a str,
    pub biz_type: Option<&'a str>,
}

// 文本审核请求
#[derive(Debug, Serialize)]
pub struct TextCheckRequest<'a> {
    pub content: &'a str,
    // 可扩展字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<&'a str>>,
}





#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SensitiveResult{
    pub label: Option<String>,
    pub sensitive_level: Option<String>,
    pub sensitive_data: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttackResult{
    pub label: Option<String>,
    pub confidence: Option<f64>,
    pub attack_level: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Advice{
    pub answer: Option<String>,
    pub hit_label: Option<String>,
    pub hit_lib_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomizedHit{
    pub lib_name: Option<String>,
    pub key_words: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Result{
    pub label: Option<String>,
    pub confidence: Option<f64>,
    pub risk_words: Option<String>,
    pub description: Option<String>,
    pub customized_hit: Option<Vec<CustomizedHit>>
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub score: Option<f64>,
    pub risk_level: Option<String>,
    pub data_id: Option<String>,
    pub sensitive_level: Option<String>,
    pub attack_level: Option<String>,
    pub manual_task_id: Option<String>,
    pub detected_language: Option<String>,
    pub translated_content: Option<String>,
    pub result: Option<Vec<Result>>,
    pub advice: Option<Vec<Advice>>,
    pub attack_result: Option<Vec<AttackResult>>,
    pub sensitive_result: Option<Vec<SensitiveResult>>,
}


/// 审核完成后阿里返回的 Json 对象，版本："2022-03-02" “1.0”
/// 主要属性是 data.result
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextCheckResponse {
    pub request_id: String,
    pub code: i32,
    pub message: String,
    pub data: Data,
}



// 图片审核请求（示例）
#[derive(Debug, Serialize)]
pub struct ImageTask<'a> {
    /// 图片 URL 或 Base64（按阿里云要求填）
    pub url: Option<&'a str>,
    pub data_base64: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<&'a str>>,
}

#[derive(Debug, Deserialize)]
pub struct ImageCheckResponse {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<serde_json::Value>,
}

// 音频/视频等可按需扩展
