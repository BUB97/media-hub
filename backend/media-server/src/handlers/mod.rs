//! 处理函数模块
//! 
//! 本模块将原来的单一 handlers.rs 文件按功能拆分为多个子模块：
//! - auth_handlers: 用户认证相关处理函数
//! - media_handlers: 媒体项目相关处理函数  
//! - system_handlers: 系统相关处理函数（健康检查、日志、监控等）
//! - cos_handlers: 腾讯云COS相关处理函数（STS临时凭证、文件上传等）

// 重新导出所有处理函数，保持向后兼容性
pub mod auth_handlers;
pub mod media_handlers;
pub mod system_handlers;
pub mod cos_handlers;

pub use auth_handlers::*;
pub use media_handlers::*;
pub use system_handlers::*;
pub use cos_handlers::*;