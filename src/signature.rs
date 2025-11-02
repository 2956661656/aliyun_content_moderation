use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// 签名抽象：将签名算法封装为 trait，便于替换
pub trait Signer: Send + Sync {
    /// 给定请求相关元素，返回待放入 Header 或 URL 的签名字符串
    fn sign(&self, method: &str, path: &str, body: &str, timestamp: &str, nonce: &str) -> Result<String>;
}

/// 一个简单的 HMAC-SHA256 签名实现。请根据阿里云文档调整具体要签名的字符串构成。
pub struct HmacSha256Signer {
    secret: String,
}

impl HmacSha256Signer {
    pub fn new(secret: impl Into<String>) -> Self {
        Self { secret: secret.into() }
    }
}

impl Signer for HmacSha256Signer {
    fn sign(&self, method: &str, path: &str, body: &str, timestamp: &str, nonce: &str) -> Result<String> {
        // 默认示例：签名基础串为 method + "\n" + path + "\n" + timestamp + nonce + body
        // **重要**：实际请依官方文档构造待签名字符串（例如含有 query、headers、AccessKeyId 等）
        let plain = format!("{}\n{}\n{}{}{}", method, path, timestamp, nonce, body);
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())?;
        mac.update(plain.as_bytes());
        let result = mac.finalize().into_bytes();
        Ok(general_purpose::STANDARD.encode(result))
    }
}
