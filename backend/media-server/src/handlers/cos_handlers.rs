use axum::{extract::Query, response::Json};
use cos_rust_sdk::sts::{GetCredentialsRequest, Policy, StsClient};
use cos_rust_sdk::{Config, CosClient, ObjectClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug)]
pub struct StsRequest {
    pub duration_seconds: Option<u32>,
    pub policy: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StsResponse {
    pub credentials: StsCredentials,
    pub expiration: String,
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StsCredentials {
    pub session_token: String,
    pub tmp_secret_id: String,
    pub tmp_secret_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StsErrorResponse {
    pub error: String,
    pub message: String,
}

/// 获取STS临时凭证
///
/// 此端点用于获取腾讯云COS的临时访问凭证，包括临时SecretId、SecretKey和SessionToken
/// 这些凭证可以用于前端直接上传文件到COS，避免在后端中转文件
#[instrument]
pub async fn get_sts_credentials(
    Query(params): Query<StsRequest>,
) -> Result<Json<StsResponse>, Json<StsErrorResponse>> {
    crate::log_with_storage!(info, "开始获取STS临时凭证");

    // 设置默认的持续时间（秒），最大7200秒（2小时）
    let duration_seconds = params.duration_seconds.unwrap_or(3600).min(7200);

    // 从环境变量获取存储桶名称
    let bucket = std::env::var("COS_BUCKET").map_err(|_| {
        crate::log_with_storage!(error, "未找到环境变量 COS_BUCKET");
        Json(StsErrorResponse {
            error: "ConfigError".to_string(),
            message: "未配置腾讯云存储桶名称".to_string(),
        })
    })?;

    // Policy 使用示例：
    // 1. Policy::allow_put_object(&bucket, Some("uploads/"))     - 仅允许上传到 uploads/ 前缀
    // 2. Policy::allow_get_object(&bucket, Some("public/"))      - 仅允许下载 public/ 前缀的文件
    // 3. Policy::allow_delete_object(&bucket, Some("temp/"))     - 仅允许删除 temp/ 前缀的文件
    // 4. Policy::allow_read_write(&bucket, Some("media/"))       - 允许读写 media/ 前缀的文件
    // 5. Policy::allow_read_write(&bucket, None)                 - 允许读写整个存储桶

    let policy_obj = if let Some(custom_policy) = params.policy.as_deref() {
        // 如果提供了自定义策略，尝试解析 JSON
        serde_json::from_str::<Policy>(custom_policy).map_err(|e| {
            crate::log_with_storage!(error, "自定义Policy解析失败: {}", e);
            Json(StsErrorResponse {
                error: "PolicyError".to_string(),
                message: format!("自定义Policy解析失败: {}", e),
            })
        })?
    } else {
        // 使用预定义的读写策略，限制在 media/ 前缀下
        // 根据需要可以改为其他策略方法：
        // Policy::allow_put_object(&bucket, Some("uploads/"))  // 仅上传
        // Policy::allow_get_object(&bucket, Some("downloads/")) // 仅下载
        // Policy::allow_delete_object(&bucket, Some("temp/"))   // 仅删除
        Policy::allow_read_write(&bucket, Some("media/"))
    };

    // 从环境变量获取配置
    let secret_id = std::env::var("COS_SECRET_ID").map_err(|_| {
        crate::log_with_storage!(error, "未找到环境变量 COS_SECRET_ID");
        Json(StsErrorResponse {
            error: "ConfigError".to_string(),
            message: "未配置腾讯云SecretId".to_string(),
        })
    })?;

    let secret_key = std::env::var("COS_SECRET_KEY").map_err(|_| {
        crate::log_with_storage!(error, "未找到环境变量 COS_SECRET_KEY");
        Json(StsErrorResponse {
            error: "ConfigError".to_string(),
            message: "未配置腾讯云SecretKey".to_string(),
        })
    })?;

    let region = std::env::var("COS_REGION").unwrap_or_else(|_| "ap-beijing".to_string());

    println!("secret_id: {}", secret_id);
    println!("secret_key: {}", secret_key);
    println!("region: {}", region);

    // 创建STS客户端
    let sts_client = StsClient::new(secret_id, secret_key, region);

    // 构建请求参数
    crate::log_with_storage!(info, "使用Policy: {:?}", policy_obj);

    let sts_request = GetCredentialsRequest {
        name: Some("media-hub-temp-credentials".to_string()),
        policy: policy_obj,
        duration_seconds: Some(duration_seconds),
    };

    crate::log_with_storage!(
        info,
        "正在请求STS临时凭证，持续时间: {}秒",
        duration_seconds
    );

    // 调用STS API获取临时凭证
    match sts_client.get_credentials(sts_request).await {
        Ok(response) => {
            crate::log_with_storage!(info, "成功获取STS临时凭证");

            // 解析响应
            let credentials = StsCredentials {
                session_token: response.token,
                tmp_secret_id: response.tmp_secret_id,
                tmp_secret_key: response.tmp_secret_key,
            };

            let expiration_time = response
                .expired_time
                .map(|t| {
                    chrono::DateTime::from_timestamp(t as i64, 0)
                        .unwrap_or_else(chrono::Utc::now)
                        .to_rfc3339()
                })
                .unwrap_or_else(|| (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339());

            let sts_response = StsResponse {
                credentials,
                expiration: expiration_time,
                request_id: "sts-request".to_string(),
            };

            Ok(Json(sts_response))
        }
        Err(e) => {
            crate::log_with_storage!(error, "获取STS临时凭证失败: {}", e);
            Err(Json(StsErrorResponse {
                error: "StsError".to_string(),
                message: format!("获取临时凭证失败: {}", e),
            }))
        }
    }
}

/// 获取COS上传配置信息
///
/// 返回COS存储桶的基本配置信息，用于前端上传文件
#[instrument]
pub async fn get_cos_config() -> Json<HashMap<String, String>> {
    crate::log_with_storage!(info, "获取COS配置信息");

    let mut config = HashMap::new();

    // 从环境变量获取COS配置
    if let Ok(bucket) = std::env::var("COS_BUCKET") {
        config.insert("bucket".to_string(), bucket);
    }

    if let Ok(region) = std::env::var("COS_REGION") {
        config.insert("region".to_string(), region);
    } else {
        config.insert("region".to_string(), "ap-beijing".to_string());
    }

    if let Ok(domain) = std::env::var("COS_DOMAIN") {
        config.insert("domain".to_string(), domain);
    }

    // 设置默认的上传路径前缀
    config.insert("upload_prefix".to_string(), "media/".to_string());

    // 设置允许的文件类型
    config.insert(
        "allowed_types".to_string(),
        "image/*,video/*,audio/*".to_string(),
    );

    // 设置最大文件大小（100MB）
    config.insert("max_file_size".to_string(), "104857600".to_string());

    Json(config)
}

/// 验证上传的文件信息
#[derive(Serialize, Deserialize, Debug)]
pub struct FileValidationRequest {
    pub filename: String,
    pub file_size: u64,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileValidationResponse {
    pub valid: bool,
    pub message: String,
    pub suggested_key: Option<String>,
}

/// 验证文件上传请求
///
/// 在获取STS凭证之前，验证文件是否符合上传要求
#[instrument]
pub async fn validate_file_upload(
    Json(request): Json<FileValidationRequest>,
) -> Json<FileValidationResponse> {
    crate::log_with_storage!(info, "验证文件上传请求: {}", request.filename);

    // 检查文件大小（最大100MB）
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
    if request.file_size > MAX_FILE_SIZE {
        return Json(FileValidationResponse {
            valid: false,
            message: format!(
                "文件大小超过限制，最大允许{}MB",
                MAX_FILE_SIZE / 1024 / 1024
            ),
            suggested_key: None,
        });
    }

    // 检查文件类型
    let allowed_types = ["image/", "video/", "audio/"];
    let is_allowed_type = allowed_types
        .iter()
        .any(|&t| request.content_type.starts_with(t));

    if !is_allowed_type {
        return Json(FileValidationResponse {
            valid: false,
            message: "不支持的文件类型，仅支持图片、视频和音频文件".to_string(),
            suggested_key: None,
        });
    }

    // 生成建议的文件键名（包含时间戳和UUID避免冲突）
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let uuid = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let extension = std::path::Path::new(&request.filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("bin");

    let suggested_key = format!(
        "media/{}_{}_{}.{}",
        timestamp,
        uuid,
        sanitize_filename(&request.filename),
        extension
    );

    crate::log_with_storage!(
        info,
        "文件验证通过: {}, 建议键名: {}",
        request.filename,
        suggested_key
    );

    Json(FileValidationResponse {
        valid: true,
        message: "文件验证通过".to_string(),
        suggested_key: Some(suggested_key),
    })
}

/// 清理文件名，移除特殊字符
fn sanitize_filename(filename: &str) -> String {
    // 移除文件扩展名
    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");

    // 只保留字母、数字、中文字符、下划线和连字符
    stem.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || (*c as u32) > 127)
        .collect::<String>()
        .chars()
        .take(50) // 限制长度
        .collect()
}

/// 从腾讯云COS删除文件
pub async fn delete_cos_file(
    cos_key: &str,
    bucket: &str,
    region: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 从环境变量获取COS配置
    let secret_id = std::env::var("COS_SECRET_ID").map_err(|_| "未找到环境变量 COS_SECRET_ID")?;

    let secret_key =
        std::env::var("COS_SECRET_KEY").map_err(|_| "未找到环境变量 COS_SECRET_KEY")?;

    // 创建COS配置
    let config = Config::new(
        &secret_id,
        &secret_key,
        &region.to_string(),
        &bucket.to_string(),
    )
    .with_timeout(Duration::from_secs(30));

    // 创建COS客户端
    let cos_client = CosClient::new(config)?;
    let object_client = ObjectClient::new(cos_client);

    // 删除对象
    match object_client.delete_object(cos_key).await {
        Ok(_) => {
            crate::log_with_storage!(info, "成功从COS删除文件: {}", cos_key);
            Ok(())
        }
        Err(e) => {
            // 如果是404错误（文件不存在），也视为成功
            if e.to_string().contains("404") || e.to_string().contains("NoSuchKey") {
                crate::log_with_storage!(info, "COS文件不存在，视为删除成功: {}", cos_key);
                Ok(())
            } else {
                Err(format!("COS删除失败: {}", e).into())
            }
        }
    }
}
