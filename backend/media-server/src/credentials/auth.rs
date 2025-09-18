use crate::credentials::jwt::verify_token;
use axum::{
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    // 首先尝试从Cookie中获取token
    let token = if let Some(cookie_header) = request.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // 解析Cookie字符串查找auth_token
            let mut auth_token = None;
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some((name, value)) = cookie.split_once('=') {
                    if name.trim() == "auth_token" {
                        auth_token = Some(value.trim());
                        break;
                    }
                }
            }

            if let Some(token) = auth_token {
                token
            } else {
                // 回退到Authorization header
                let auth_header = request
                    .headers()
                    .get(AUTHORIZATION)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header| header.strip_prefix("Bearer "));

                match auth_header {
                    Some(token) => token,
                    None => return Err(StatusCode::UNAUTHORIZED),
                }
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        // 回退到Authorization header
        let auth_header = request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "));

        match auth_header {
            Some(token) => token,
            None => return Err(StatusCode::UNAUTHORIZED),
        }
    };

    let claims = match verify_token(token) {
        Ok(token_data) => token_data.claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let auth_user = AuthUser {
        user_id: claims.sub,
        username: claims.username,
    };

    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}
