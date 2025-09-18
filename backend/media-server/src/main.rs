// 添加模块声明
mod credentials;
mod database;
mod handlers;
mod logging;
mod routes;

use database::Database;
use logging::init_logging;
use routes::{create_routes, print_endpoints};

/// Media Hub 服务器入口点
#[tokio::main]
async fn main() {
    // 加载.env文件
    dotenv::dotenv().ok();

    // 初始化日志系统
    init_logging().expect("日志初始化失败");

    // 初始化数据库
    let database = Database::new().await.expect("数据库初始化失败");

    // 创建应用路由
    let app = create_routes().with_state(database);

    // 绑定服务器地址
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    // 打印启动信息
    println!("🚀 Media Hub Server started on http://0.0.0.0:8000");
    print_endpoints();

    // 记录服务启动日志
    crate::log_with_storage!(info, "服务已启动");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
