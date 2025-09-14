use axum::{
    extract::Extension,
    http::StatusCode,
    response::Json,
};
use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
use chrono::Utc;
use tracing::instrument;

// use crate::credentials::user::*;
// use crate::credentials::jwt::*;
use crate::credentials::*;

// 注册端点
#[instrument]
pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<Json<UserInfo>, (StatusCode, String)> {
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
pub async fn login(Json(payload): Json<LoginRequest>) -> Result<(HeaderMap, Json<LoginResponse>), (StatusCode, String)> {
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
pub async fn me(Extension(auth_user): Extension<AuthUser>) -> Result<Json<UserInfo>, StatusCode> {
    let storage = USER_STORAGE.read().await;
    let user = storage.get(&auth_user.user_id)
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