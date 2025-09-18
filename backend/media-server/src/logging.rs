use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

// 全局日志存储
pub static LOG_STORAGE: RwLock<Vec<LogEntry>> = RwLock::const_new(Vec::new());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub module: String,
    pub fields: HashMap<String, String>,
}

// 初始化日志系统
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // 创建文件输出器，按天轮转
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "media-server.log");

    // 创建 JSON 格式的文件层
    let file_layer = fmt::layer()
        .json()
        .with_writer(file_appender)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // 创建控制台输出层
    let console_layer = fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // 环境过滤器 - 默认显示所有级别的日志
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));

    // 组合所有层
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    info!("日志系统初始化完成");
    Ok(())
}

// 存储日志条目到内存
pub async fn store_log_entry(
    level: String,
    message: String,
    user_id: Option<String>,
    request_id: Option<String>,
) {
    let entry = LogEntry {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        level: level.clone(),
        message,
        user_id,
        request_id,
        module: "media_server".to_string(),
        fields: HashMap::new(),
    };

    let mut storage = LOG_STORAGE.write().await;
    storage.push(entry);

    // 保持最近 1000 条日志
    let len = storage.len();
    if len > 1000 {
        storage.drain(0..len - 1000);
    }
}

// 日志宏，同时输出到 tracing 和存储
#[macro_export]
macro_rules! log_with_storage {
    (error, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            tracing::error!("{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    "ERROR".to_string(),
                    message,
                    None,
                    None,
                ).await;
            });
        }
    };
    (warn, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            tracing::warn!("{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    "WARN".to_string(),
                    message,
                    None,
                    None,
                ).await;
            });
        }
    };
    (info, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            tracing::info!("{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    "INFO".to_string(),
                    message,
                    None,
                    None,
                ).await;
            });
        }
    };
    (debug, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            tracing::debug!("{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    "DEBUG".to_string(),
                    message,
                    None,
                    None,
                ).await;
            });
        }
    };
    (trace, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            tracing::trace!("{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    "TRACE".to_string(),
                    message,
                    None,
                    None,
                ).await;
            });
        }
    };
}

// 带用户信息的日志宏
#[macro_export]
macro_rules! log_with_user {
    ($level:ident, $user_id:expr, $request_id:expr, $($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            let level_str = stringify!($level).to_uppercase();
            tracing::$level!(user_id = %$user_id, request_id = %$request_id, "{}", message);
            tokio::spawn(async move {
                $crate::logging::store_log_entry(
                    level_str,
                    message,
                    Some($user_id.to_string()),
                    Some($request_id.to_string()),
                ).await;
            });
        }
    };
}
