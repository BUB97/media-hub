use axum::{
    extract::{Query, Extension, Request},
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
use axum::response::Response;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use axum::http::{Method, HeaderName};
use tracing::{info, instrument};
use uuid::Uuid;

mod logging;
use logging::{LOG_STORAGE, LogEntry, init_logging};

#[derive(Serialize, Deserialize, Debug)]
struct LogQueryParams {
    user_id: Option<String>,
    level: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetricsResponse {}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    message: String,
    timestamp: DateTime<Utc>,
    request_id: String,
}

#[derive(Serialize, Deserialize)]
struct MediaItem {
    id: u32,
    title: String,
    description: String,
    media_type: String,
}

#[derive(Deserialize)]
struct CreateMediaRequest {
    title: String,
    description: String,
    media_type: String,
}

// 查询日志端点
#[instrument]
async fn query_logs(Query(params): Query<LogQueryParams>) -> Json<Vec<LogEntry>> {
    let storage = LOG_STORAGE.read().await;
    let mut filtered_logs: Vec<LogEntry> = storage.clone();
    
    // 按用户ID过滤
    if let Some(user_id) = &params.user_id {
        filtered_logs.retain(|log| {
            log.user_id.as_ref().map_or(false, |id| id == user_id)
        });
    }
    
    // 按日志级别过滤
    if let Some(level) = &params.level {
        filtered_logs.retain(|log| log.level.to_lowercase() == level.to_lowercase());
    }
    
    // 按时间范围过滤
    if let Some(start_time) = &params.start_time {
        if let Ok(start) = DateTime::parse_from_rfc3339(start_time) {
            filtered_logs.retain(|log| log.timestamp >= start.with_timezone(&Utc));
        }
    }
    
    if let Some(end_time) = &params.end_time {
        if let Ok(end) = DateTime::parse_from_rfc3339(end_time) {
            filtered_logs.retain(|log| log.timestamp <= end.with_timezone(&Utc));
        }
    }
    
    // 限制返回数量
    let limit = params.limit.unwrap_or(100).min(1000);
    filtered_logs.truncate(limit);
    
    // 按时间倒序排列
    filtered_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    crate::log_with_storage!(info, "日志查询完成，返回 {} 条记录", filtered_logs.len());
    Json(filtered_logs)
}

// 监控指标端点
#[instrument]
async fn metrics() -> Json<MetricsResponse> {
    Json(MetricsResponse {
    })
}

// 健康检查端点
#[instrument]
async fn health() -> Json<HealthResponse> {
    let request_id = Uuid::new_v4().to_string();
    let user_id = "demo_user_123";
    let request_id_for_log = request_id.clone();
    
    // 使用 log_with_user 宏来演示 user_id 和 request_id 字段
    crate::log_with_user!(info, user_id, request_id_for_log, "健康检查请求成功，用户: {}, 请求ID: {}", user_id, request_id_for_log);
    
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Media Hub Server is running".to_string(),
        timestamp: Utc::now(),
        request_id,
    })
}

// 获取所有媒体项目
async fn get_media() -> Json<Vec<MediaItem>> {
    crate::log_with_storage!(info, "开始获取所有媒体项目");
    
    // 这里返回模拟数据，实际项目中会从数据库获取
    let media_items = vec![
        MediaItem {
            id: 1,
            title: "Sample Video".to_string(),
            description: "A sample video file".to_string(),
            media_type: "video".to_string(),
        },
        MediaItem {
            id: 2,
            title: "Sample Audio".to_string(),
            description: "A sample audio file".to_string(),
            media_type: "audio".to_string(),
        },
    ];
    
    crate::log_with_storage!(debug, "返回 {} 个媒体项目", media_items.len());
    Json(media_items)
}

// 创建新的媒体项目
async fn create_media(Json(payload): Json<CreateMediaRequest>) -> (StatusCode, Json<MediaItem>) {
    crate::log_with_storage!(info, "创建新媒体项目: {}", payload.title);
    
    // 这里是模拟创建，实际项目中会保存到数据库
    let new_media = MediaItem {
        id: 3, // 实际中应该是自动生成的ID
        title: payload.title.clone(),
        description: payload.description,
        media_type: payload.media_type,
    };
    
    crate::log_with_storage!(debug, "成功创建媒体项目，ID: {}", new_media.id);
    (StatusCode::CREATED, Json(new_media))
}

// 根据查询参数搜索媒体
async fn search_media(Query(params): Query<HashMap<String, String>>) -> Json<Vec<MediaItem>> {
    let search_term = params.get("q").unwrap_or(&String::new()).to_lowercase();
    
    // 模拟搜索逻辑
    let all_media = vec![
        MediaItem {
            id: 1,
            title: "Sample Video".to_string(),
            description: "A sample video file".to_string(),
            media_type: "video".to_string(),
        },
        MediaItem {
            id: 2,
            title: "Sample Audio".to_string(),
            description: "A sample audio file".to_string(),
            media_type: "audio".to_string(),
        },
    ];
    
    let filtered_media: Vec<MediaItem> = all_media
        .into_iter()
        .filter(|item| {
            item.title.to_lowercase().contains(&search_term)
                || item.description.to_lowercase().contains(&search_term)
        })
        .collect();
    
    Json(filtered_media)
}

// 添加新的模块
mod user;
mod jwt;
mod auth;

use user::*;
use jwt::*;
use auth::*;

// 注册端点
#[instrument]
async fn register(Json(payload): Json<RegisterRequest>) -> Result<Json<UserInfo>, (StatusCode, String)> {
    // 检查用户名是否已存在
    let username_index = USERNAME_INDEX.read().await;
    if username_index.contains_key(&payload.username) {
        return Err((StatusCode::CONFLICT, "用户名已存在".to_string()));
    }
    drop(username_index);
    
    // 创建新用户
    let user = User::new(payload.username.clone(), payload.email, payload.password)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let user_info = user.to_user_info();
    let user_id = user.id.clone();
    
    // 存储用户
    let mut storage = USER_STORAGE.write().await;
    let mut username_index = USERNAME_INDEX.write().await;
    
    storage.insert(user_id.clone(), user);
    username_index.insert(payload.username, user_id);
    
    crate::log_with_storage!(info, "新用户注册: {}", user_info.username);
    
    Ok(Json(user_info))
}

// 登录端点
async fn login(Json(payload): Json<LoginRequest>) -> Result<(HeaderMap, Json<LoginResponse>), (StatusCode, String)> {
    // 查找用户
    let username_index = USERNAME_INDEX.read().await;
    let user_id = username_index.get(&payload.username)
        .ok_or((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()))?;
    let user_id = user_id.clone();
    drop(username_index);
    
    let (token, user_info, user_id_for_log, username_for_log) = {
        let mut storage = USER_STORAGE.write().await;
        let user = storage.get_mut(&user_id)
            .ok_or((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()))?;
        
        // 验证密码
        if !user.verify_password(&payload.password) {
            return Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()));
        }
        
        // 更新最后登录时间
        user.last_login = Some(Utc::now());
        
        // 创建 JWT token
        let token = create_token(user.id.clone(), user.username.clone())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        
        let user_info = user.to_user_info();
        let user_id_for_log = user.id.clone();
        let username_for_log = user.username.clone();
        
        (token, user_info, user_id_for_log, username_for_log)
    };
    
    let expires_at = Utc::now() + chrono::Duration::hours(24);
    
    // 创建HttpOnly Cookie
    let cookie_value = format!(
        "auth_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Strict",
        token,
        24 * 3600 // 24小时
    );

    
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie_value).unwrap());
    
    crate::log_with_user!(info, &user_id_for_log, &user_id_for_log, "用户登录: {}", username_for_log);
    
    Ok((headers, Json(LoginResponse {
        user: user_info,
        expires_at,
    })))
}

// 获取当前用户信息 (需要认证)
async fn me(Extension(auth_user): Extension<AuthUser>) -> Result<Json<UserInfo>, StatusCode> {
    let storage = USER_STORAGE.read().await;
    let user = storage.get(&auth_user.user_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user.to_user_info()))
}

// 登出端点 (清除Cookie)
async fn logout(Extension(auth_user): Extension<AuthUser>) -> (HeaderMap, Json<serde_json::Value>) {
    crate::log_with_user!(info, &auth_user.user_id, &auth_user.user_id, "用户登出: {}", auth_user.username);
    
    // 清除Cookie
    let cookie_value = "auth_token=; HttpOnly; Path=/; Max-Age=0; SameSite=None";
    
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(cookie_value).unwrap());
    
    (headers, Json(serde_json::json!({
        "message": "登出成功"
    })))
}

// 在 main 函数中更新路由
#[tokio::main]
async fn main() {
    init_logging().expect("日志初始化失败");
    
    // 公开路由 (不需要认证)
    let public_routes = Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login));
    
    // 需要认证的路由
    let protected_routes = Router::new()
        .route("/api/auth/me", get(me))
        .route("/api/auth/logout", post(logout))
        .route("/api/media", get(get_media))
        .route("/api/media", post(create_media))
        .route("/api/media/search", get(search_media))
        .route("/api/logs", get(query_logs))
        .route("/api/metrics", get(metrics))
        .layer(middleware::from_fn(auth_middleware));
    
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes);
        // 配置 CORS 以支持 Cookie 认证
        // .layer(CorsLayer::new()
        //         .allow_origin("http://localhost".parse::<axum::http::HeaderValue>().unwrap())
        //         .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        //         .allow_headers([
        //             HeaderName::from_static("content-type"),
        //             HeaderName::from_static("authorization"),
        //             HeaderName::from_static("accept"),
        //         ])
        //         .allow_credentials(true)
        // );
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    
    println!("🚀 Media Hub Server started on http://0.0.0.0:3000");
    println!("📋 Available endpoints:");
    println!("  GET  /health              - 健康检查");
    println!("  POST /auth/register       - 用户注册");
    println!("  POST /auth/login          - 用户登录");
    println!("  GET  /auth/me             - 获取当前用户信息 (需要认证)");
    println!("  POST /auth/logout         - 用户登出 (需要认证)");
    println!("  GET  /api/media           - 获取所有媒体 (需要认证)");
    println!("  POST /api/media           - 创建新媒体 (需要认证)");
    println!("  GET  /api/media/search    - 搜索媒体 (需要认证)");
    println!("  GET  /api/logs            - 查询日志记录 (需要认证)");
    println!("  GET  /api/metrics         - 获取监控指标 (需要认证)");
    
    crate::log_with_storage!(info, "服务已启动");
    axum::serve(listener, app).await.unwrap();
}
