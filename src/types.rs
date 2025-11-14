use serde::{Deserialize, Serialize};
use serde_json::Value;

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
// #[derive(Debug, Serialize)]
// pub struct TextCheckRequest<'a> {
//     pub content: &'a str,
//     // 可扩展字段
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub scenes: Option<Vec<&'a str>>,
// }





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
pub struct TextCheckResult {
    pub label: Option<String>,
    pub confidence: Option<f64>,
    pub risk_words: Option<String>,
    pub description: Option<String>,
    pub customized_hit: Option<Vec<CustomizedHit>>
}

///使用建议： 做好判断再使用详细字段
///    if  data.label.is_some() &&
///        data.label.unwrap().as_str() != "nonLabel" &&
///        data.[other_fields].unwrap() {}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextCheckData {
    pub score: Option<f64>,
    pub risk_level: Option<String>,
    pub data_id: Option<String>,
    pub sensitive_level: Option<String>,
    pub attack_level: Option<String>,
    pub manual_task_id: Option<String>,
    pub detected_language: Option<String>,
    pub translated_content: Option<String>,
    pub result: Option<Vec<TextCheckResult>>,
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
    pub data: Option<TextCheckData>,
}



// 图片审核请求
// #[derive(Debug, Serialize)]
// pub struct ImageTask<'a> {
//     /// 图片 URL 或 Base64（按阿里云要求填）
//     pub url: Option<&'a str>,
//     pub data_base64: Option<&'a str>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub scenes: Option<Vec<&'a str>>,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageCheckResult {
    pub label: Option<String>,
    pub confidence: Option<f64>,
    pub description: Option<String>,
    pub risk_level: Option<String>,
}

///ext 内容太多且作者用不上，你们可以自己扩展需要的，或者直接使用"[]"运算符获取。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageCheckData {
    pub data_id: Option<String>,
    pub result: Option<Vec<ImageCheckResult>>,
    pub risk_level: Option<String>,
    pub ext: Option<Value>,
    pub manual_task_id: Option<String>,

}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageCheckResponse {
    pub request_id: String,
    pub code: i32,
    pub msg: String,
    pub data: Option<ImageCheckData>,
}

// 音频/视频等可按需扩展
