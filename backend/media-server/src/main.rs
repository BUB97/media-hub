use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tracing::instrument;
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

#[tokio::main]
async fn main() {
    // 初始化日志系统
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    // 创建 CORS 层
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建应用路由
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/media", get(get_media))
        .route("/api/media", post(create_media))
        .route("/api/media/search", get(search_media))
        .route("/api/logs", get(query_logs))
        .route("/api/metrics", get(metrics))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Media Hub Server started on http://0.0.0.0:3000");
    println!("📋 Available endpoints:");
    println!("  GET  /health           - 健康检查");
    println!("  GET  /api/media       - 获取所有媒体");
    println!("  POST /api/media       - 创建新媒体");
    println!("  GET  /api/media/search?q=<term> - 搜索媒体");
    println!("  GET  /api/logs         - 查询日志记录");
    println!("  GET  /api/metrics      - 获取监控指标");

    crate::log_with_storage!(info, "服务已启动");
    axum::serve(listener, app).await.unwrap();
}
