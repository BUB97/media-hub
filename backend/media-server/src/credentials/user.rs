use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

// 数据库操作函数
pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(pool: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, created_at, last_login, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.created_at)
        .bind(&user.last_login)
        .bind(&user.is_active)
        .execute(pool)
        .await?;

        info!("用户创建成功: {}", user.username);
        Ok(())
    }

    pub async fn find_by_username(
        pool: &Pool<Postgres>,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at, last_login, is_active FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at, last_login, is_active FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update_last_login(
        pool: &Pool<Postgres>,
        user_id: &str,
        last_login: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET last_login = $1 WHERE id = $2")
            .bind(&last_login)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn username_exists(
        pool: &Pool<Postgres>,
        username: &str,
    ) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn email_exists(pool: &Pool<Postgres>, email: &str) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(count > 0)
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Result<Self, String> {
        let password_hash =
            bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|_| "密码加密失败")?;

        Ok(User {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            created_at: Utc::now(),
            last_login: None,
            is_active: true,
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }

    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at,
            last_login: self.last_login,
        }
    }
}
