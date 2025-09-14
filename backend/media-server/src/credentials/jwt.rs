use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

// JWT 密钥 (生产环境应从环境变量读取)
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 用户ID
    pub username: String,
    pub exp: i64,     // 过期时间
    pub iat: i64,     // 签发时间
}

impl Claims {
    pub fn new(user_id: String, username: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // 24小时过期
        
        Claims {
            sub: user_id,
            username,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_token(user_id: String, username: String) -> Result<String, String> {
    let claims = Claims::new(user_id, username);
    let header = Header::default();
    let key = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    
    encode(&header, &claims, &key)
        .map_err(|_| "Token 创建失败".to_string())
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>, String> {
    let key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let validation = Validation::default();
    
    decode::<Claims>(token, &key, &validation)
        .map_err(|_| "Token 验证失败".to_string())
}