use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};

use crate::database::Database;

use crate::handlers::*;
use crate::credentials::auth_middleware;

/// 创建应用程序的所有路由
pub fn create_routes() -> Router<Database> {
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
        .route("/api/media/{id}", get(get_media_by_id))
        .route("/api/media/{id}", put(update_media))
        .route("/api/media/{id}", delete(delete_media))
        .route("/api/media/{id}/upload", put(upload_media_file))
        // AI 分析相关路由
        .route("/api/ai/analysis", post(ai_handlers::create_analysis))
        .route("/api/ai/analysis/{id}", get(ai_handlers::get_analysis))
        .route("/api/ai/analysis/{id}", delete(ai_handlers::delete_analysis))
        .route("/api/ai/media/{media_id}/analyses", get(ai_handlers::get_media_analyses))
        .route("/api/ai/search/similarity", post(ai_handlers::similarity_search))
        .route("/api/ai/stats", get(ai_handlers::get_analysis_stats))
        .route("/api/logs", get(query_logs))
        .route("/api/metrics", get(metrics))
        .route("/api/cos/sts", get(get_sts_credentials))
        .route("/api/cos/config", get(get_cos_config))
        .route("/api/cos/validate", post(validate_file_upload))
        .layer(middleware::from_fn(auth_middleware));
    
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
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
        // )
}

/// 打印所有可用的API端点
pub fn print_endpoints() {
    println!("📋 Available endpoints:");
    println!("  GET  /api/health          - 健康检查");
    println!("  POST /api/auth/register   - 用户注册");
    println!("  POST /api/auth/login      - 用户登录");
    println!("  GET  /api/auth/me         - 获取当前用户信息 (需要认证)");
    println!("  POST /api/auth/logout     - 用户登出 (需要认证)");
    println!("  GET  /api/media           - 获取用户媒体列表 (需要认证)");
    println!("  POST /api/media           - 创建新媒体 (需要认证)");
    println!("  GET  /api/media/search    - 搜索媒体 (需要认证)");
    println!("  GET  /api/media/:id       - 获取单个媒体 (需要认证)");
    println!("  PUT  /api/media/:id       - 更新媒体信息 (需要认证)");
    println!("  DELETE /api/media/:id     - 删除媒体 (需要认证)");
    println!("  PUT  /api/media/:id/upload - 上传媒体文件 (需要认证)");
    println!("  POST /api/ai/analysis     - 创建AI分析任务 (需要认证)");
    println!("  GET  /api/ai/analysis/:id - 获取AI分析结果 (需要认证)");
    println!("  DELETE /api/ai/analysis/:id - 删除AI分析结果 (需要认证)");
    println!("  GET  /api/ai/media/:media_id/analyses - 获取媒体的所有分析结果 (需要认证)");
    println!("  POST /api/ai/search/similarity - 相似度搜索 (需要认证)");
    println!("  GET  /api/ai/stats        - 获取AI分析统计信息 (需要认证)");
    println!("  GET  /api/logs            - 查询日志记录 (需要认证)");
    println!("  GET  /api/metrics         - 获取监控指标 (需要认证)");
    println!("  GET  /api/cos/sts         - 获取COS STS临时凭证 (需要认证)");
    println!("  GET  /api/cos/config      - 获取COS配置信息 (需要认证)");
    println!("  POST /api/cos/validate    - 验证文件上传参数 (需要认证)");
}