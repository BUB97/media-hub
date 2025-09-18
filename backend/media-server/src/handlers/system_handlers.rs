use axum::{extract::Query, response::Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::logging::{LOG_STORAGE, LogEntry};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogQueryParams {
    pub user_id: Option<String>,
    pub level: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsResponse {}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}

// 查询日志端点
#[instrument]
pub async fn query_logs(Query(params): Query<LogQueryParams>) -> Json<Vec<LogEntry>> {
    let storage = LOG_STORAGE.read().await;
    let mut filtered_logs: Vec<LogEntry> = storage.clone();

    // 按用户ID过滤
    if let Some(user_id) = &params.user_id {
        filtered_logs.retain(|log| log.user_id.as_ref().map_or(false, |id| id == user_id));
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
pub async fn metrics() -> Json<MetricsResponse> {
    Json(MetricsResponse {})
}

// 健康检查端点
#[instrument]
pub async fn health() -> Json<HealthResponse> {
    let request_id = Uuid::new_v4().to_string();
    let user_id = "demo_user_123";
    let request_id_for_log = request_id.clone();

    // 使用 log_with_user 宏来演示 user_id 和 request_id 字段
    crate::log_with_user!(
        info,
        user_id,
        request_id_for_log,
        "健康检查请求成功，用户: {}, 请求ID: {}",
        user_id,
        request_id_for_log
    );

    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Media Hub Server is running".to_string(),
        timestamp: Utc::now(),
        request_id,
    })
}
