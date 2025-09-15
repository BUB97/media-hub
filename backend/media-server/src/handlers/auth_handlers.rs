use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
use chrono::Utc;
use tracing::instrument;

use crate::credentials::*;
use crate::database::Database;

// 注册端点
#[instrument]
pub async fn register(
    State(database): State<Database>,
    Json(payload): Json<RegisterRequest>
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let pool = database.get_pool();
    
    // 检查用户名是否已存在
    if UserRepository::username_exists(pool, &payload.username).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误: {}", e)))? {
        return Err((StatusCode::CONFLICT, "用户名已存在".to_string()));
    }
    
    // 检查邮箱是否已存在
    if UserRepository::email_exists(pool, &payload.email).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误: {}", e)))? {
        return Err((StatusCode::CONFLICT, "邮箱已存在".to_string()));
    }
    
    // 创建新用户
    let user = User::new(payload.username.clone(), payload.email, payload.password)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let user_info = user.to_user_info();
    
    // 存储用户到数据库
    UserRepository::create_user(pool, &user).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("用户创建失败: {}", e)))?;
    
    crate::log_with_storage!(info, "新用户注册: {}", user_info.username);
    
    Ok(Json(user_info))
}

// 登录端点
pub async fn login(
    State(database): State<Database>,
    Json(payload): Json<LoginRequest>
) -> Result<(HeaderMap, Json<LoginResponse>), (StatusCode, String)> {
    let pool = database.get_pool();
    
    // 查找用户
    let mut user = UserRepository::find_by_username(pool, &payload.username).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误: {}", e)))?
        .ok_or((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()))?;
    
    // 验证密码
    if !user.verify_password(&payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()));
    }
    
    // 更新最后登录时间
    let now = Utc::now();
    user.last_login = Some(now);
    UserRepository::update_last_login(pool, &user.id, now).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("更新登录时间失败: {}", e)))?;
    
    // 创建 JWT token
    let token = create_token(user.id.clone(), user.username.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    let user_info = user.to_user_info();
    let user_id_for_log = user.id.clone();
    let username_for_log = user.username.clone();
    
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
pub async fn me(
    State(database): State<Database>,
    Extension(auth_user): Extension<AuthUser>
) -> Result<Json<UserInfo>, StatusCode> {
    let pool = database.get_pool();
    
    let user = UserRepository::find_by_id(pool, &auth_user.user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(user.to_user_info()))
}

// 登出端点 (清除Cookie)
pub async fn logout(Extension(auth_user): Extension<AuthUser>) -> (HeaderMap, Json<serde_json::Value>) {
    crate::log_with_user!(info, &auth_user.user_id, &auth_user.user_id, "用户登出: {}", auth_user.username);
    
    // 清除Cookie
    let cookie_value = "auth_token=; HttpOnly; Path=/; Max-Age=0; SameSite=None";
    
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(cookie_value).unwrap());
    
    (headers, Json(serde_json::json!({
        "message": "登出成功"
    })))
}