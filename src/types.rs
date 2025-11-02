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
    label: Option<String>,
    sensitive_level: Option<String>,
    sensitive_data: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttackResult{
    label: Option<String>,
    confidence: Option<f64>,
    attack_level: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Advice{
    answer: Option<String>,
    hit_label: Option<String>,
    hit_lib_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomizedHit{
    lib_name: Option<String>,
    key_words: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Result{
    label: Option<String>,
    confidence: Option<f64>,
    risk_words: Option<String>,
    description: Option<String>,
    customized_hit: Option<Vec<CustomizedHit>>
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    score: Option<f64>,
    risk_level: Option<String>,
    data_id: Option<String>,
    sensitive_level: Option<String>,
    attack_level: Option<String>,
    manual_task_id: Option<String>,
    detected_language: Option<String>,
    translated_content: Option<String>,
    result: Option<Vec<Result>>,
    advice: Option<Vec<Advice>>,
    attack_result: Option<Vec<AttackResult>>,
    sensitive_result: Option<Vec<SensitiveResult>>,
}


/// 审核完成后阿里返回的 Json 对象，版本："2022-03-02" “1.0”
/// 主要属性是 data.result
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextCheckResponse {
    request_id: Option<String>,
    code: Option<i32>,
    message: Option<String>,
    data: Option<Data>,
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
