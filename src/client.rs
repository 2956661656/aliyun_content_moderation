use std::str::FromStr;
use crate::error::ModerationError;
use crate::types::*;
use anyhow::Result;
use reqwest::Client as HttpClient;
use hmac::{Hmac, Mac};
use log::error;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use sha1::Sha1; // 或者 sha2::Sha256

/// 发起审核请求的客户端。
/// 建议将access_key保存到配置文件，从配置文件读取。
pub struct ModerationClient {
    pub http: HttpClient,
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub version: String,
}

impl ModerationClient {
    /// 创建 client
    ///
    /// `endpoint` 示例：`https://green-cip.cn-shanghai.aliyuncs.com`。具体需要参考阿里云文档：`https://help.aliyun.com/document_detail/433945.html?scm=20140722.H_433945._.OR_help-T_cn~zh-V_1`
    /// `version` 示例：`2022-03-02`
    /// 建议将access_key保存到配置文件，从配置文件读取。
    pub fn new(endpoint: impl Into<String>,
               access_key_id: impl Into<String>,
               access_key_secret: impl Into<String>,
               version: impl Into<String>,
    ) -> Self {
        ModerationClient {
            http: HttpClient::new(),
            endpoint: endpoint.into(),
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            version: version.into()
        }
    }

    fn build_query_params(&self, action: &str, scene: &str, parameters: &str)->anyhow::Result<String>{
        // timestamp + nonce
        let timestamp = chrono::Utc::now().to_rfc3339();
        let nonce = uuid::Uuid::new_v4().to_string();

        const URL_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
            .remove(b'-')
            .remove(b'_')
            .remove(b'.')
            .remove(b'~')
        ;


        let canonicalized_query_string =
            format!("AccessKeyId={}&Action={}&Format=JSON&Service={}&ServiceParameters={}&SignatureMethod=Hmac-SHA1&SignatureNonce={}&SignatureVersion={}&Timestamp={}&Version={}",
            utf8_percent_encode(&self.access_key_id, URL_ENCODE_SET),
            utf8_percent_encode(action, URL_ENCODE_SET),
            utf8_percent_encode(scene, URL_ENCODE_SET),
            utf8_percent_encode(parameters, URL_ENCODE_SET),
            utf8_percent_encode(&nonce, URL_ENCODE_SET),
            utf8_percent_encode("1.0", URL_ENCODE_SET),
            utf8_percent_encode(&timestamp, URL_ENCODE_SET),
            utf8_percent_encode(&self.version, URL_ENCODE_SET),
        );

        let percent_encode_canonicalized_query_string = utf8_percent_encode(&canonicalized_query_string, URL_ENCODE_SET).to_string();

        let string_to_sign =
            String::from_str("POST")? + "&" +
                utf8_percent_encode("/", URL_ENCODE_SET).to_string().as_str()
                + "&" + percent_encode_canonicalized_query_string.as_str();

        let mut mac = Hmac::<Sha1>::new_from_slice(
            (self.access_key_secret.clone()+"&")
                .as_bytes())?;
        // 2. 输入待签名内容
        mac.update(string_to_sign.as_bytes());
        // 3. 计算出 HMAC 值（签名）
        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        let signature = base64::encode(code_bytes);

        let signature_encoded = utf8_percent_encode(&signature, URL_ENCODE_SET);

        let sign_query = format!("&Signature={signature_encoded}");

        Ok(canonicalized_query_string+sign_query.as_str())
    }

    /// 文本审核
    /// `scene`: 使用场景。示例：`nickname_detection_pro`，具体需要参考阿里云文档：`https://help.aliyun.com/document_detail/464388.html?scm=20140722.H_464388._.OR_help-T_cn~zh-V_1`
    /// `content`: 具体需要审核的文本内容。
    pub async fn check_text(&self, scene: &str, content: &str) -> Result<TextCheckResponse, ModerationError> {
        let content = format!("{{ \"content\": \"{content}\" }}");
        // 签名
        let query = self.build_query_params("TextModerationPlus", scene, &content);

        let query = match query {
            Ok(data) => data,
            Err(e) => return Err(ModerationError::Parse(format!("构建查询参数时发生错误: {}", e)))
        };

        let url = format!("{}?{}", self.endpoint, query);

        let request = self.http.post(url);

        let resp = request.send().await.map_err(|e|{
            error!("发送审核请求时发生错误：{}，如果是 builder error，请检查url是否添加https://协议", e.to_string());
            ModerationError::Http(e.to_string())
        })?;
        let status = resp.status();
        let text = resp.text().await.map_err(|e| ModerationError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(ModerationError::Http(format!("status {} body {}", status, text)));
        }
        let parsed: TextCheckResponse = serde_json::from_str(&text).map_err(|e|
            ModerationError::Parse(e.to_string())
        )?;
        Ok(parsed)
    }

    // 图片检查（示例）
    // pub async fn check_image(&self, tasks: Vec<ImageTask<'_>>) -> Result<ImageCheckResponse, ModerationError> {
    //
    // }

    // TODO: check_audio, check_video, 支持分片、上传到 OSS 后回调检查等
}
