use axum::{
    extract::{Query, State, Path, Extension},
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::database::Database;
use crate::credentials::{Claims, AuthUser};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct MediaItem {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub content_type: String,
    pub cos_key: String,
    pub cos_url: String,
    pub cos_bucket: String,
    pub cos_region: String,
    pub media_type: String,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateMediaRequest {
    pub title: String,
    pub description: Option<String>,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub content_type: String,
    pub cos_key: String,
    pub cos_url: String,
    pub cos_bucket: String,
    pub cos_region: String,
    pub media_type: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateMediaRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct MediaListResponse {
    pub items: Vec<MediaItem>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Deserialize, Debug)]
pub struct MediaQueryParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub media_type: Option<String>,
    pub q: Option<String>,
}

/// 获取用户的媒体项目
pub async fn get_media(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<MediaQueryParams>,
) -> Result<Json<MediaListResponse>, StatusCode> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * per_page;
    
    let mut query = "SELECT * FROM media_files WHERE user_id = $1 AND status = 'active'".to_string();
    let mut query_params = vec![auth_user.user_id.clone()];
    let mut param_count = 1;
    
    // 添加媒体类型过滤
    if let Some(media_type) = &params.media_type {
        param_count += 1;
        query.push_str(&format!(" AND media_type = ${}", param_count));
        query_params.push(media_type.clone());
    }
    
    // 添加搜索过滤
    if let Some(search) = &params.q {
        param_count += 1;
        query.push_str(&format!(" AND (title ILIKE ${} OR description ILIKE ${})", param_count, param_count));
        query_params.push(format!("%{}%", search));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    
    // 获取总数
    let count_query = query.replace("SELECT *", "SELECT COUNT(*)").replace(" ORDER BY created_at DESC", "");
    let total: i64 = match sqlx::query_scalar(&count_query)
        .bind(&auth_user.user_id)
        .fetch_one(&db.pool)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Database error getting media count: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // 添加分页
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));
    
    let rows = match sqlx::query_as::<_, MediaItem>(&query)
        .bind(&auth_user.user_id)
        .fetch_all(&db.pool)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Database error getting media: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(MediaListResponse {
        items: rows,
        total,
        page,
        per_page,
    }))
}

/// 创建新的媒体项目
pub async fn create_media(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    AxumJson(payload): AxumJson<CreateMediaRequest>,
) -> Result<Json<MediaItem>, StatusCode> {
    let media_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let media_item = MediaItem {
        id: media_id.clone(),
        user_id: auth_user.user_id.clone(),
        title: payload.title,
        description: payload.description,
        filename: payload.filename,
        original_filename: payload.original_filename,
        file_size: payload.file_size,
        content_type: payload.content_type,
        cos_key: payload.cos_key,
        cos_url: payload.cos_url,
        cos_bucket: payload.cos_bucket,
        cos_region: payload.cos_region,
        media_type: payload.media_type,
        status: "active".to_string(),
        metadata: payload.metadata,
        created_at: now,
        updated_at: now,
    };
    
    let query = r#"
        INSERT INTO media_files (
            id, user_id, title, description, filename, original_filename,
            file_size, content_type, cos_key, cos_url, cos_bucket, cos_region,
            media_type, status, metadata, created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
        )
    "#;
    
    match sqlx::query(query)
        .bind(&media_item.id)
        .bind(&media_item.user_id)
        .bind(&media_item.title)
        .bind(&media_item.description)
        .bind(&media_item.filename)
        .bind(&media_item.original_filename)
        .bind(&media_item.file_size)
        .bind(&media_item.content_type)
        .bind(&media_item.cos_key)
        .bind(&media_item.cos_url)
        .bind(&media_item.cos_bucket)
        .bind(&media_item.cos_region)
        .bind(&media_item.media_type)
        .bind(&media_item.status)
        .bind(&media_item.metadata)
        .bind(&media_item.created_at)
        .bind(&media_item.updated_at)
        .execute(&db.pool)
        .await
    {
        Ok(_) => Ok(Json(media_item)),
        Err(e) => {
            eprintln!("Database error creating media: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 搜索媒体项目
pub async fn search_media(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<MediaQueryParams>,
) -> Result<Json<MediaListResponse>, StatusCode> {
    // 重用 get_media 函数的逻辑，因为它已经包含了搜索功能
    get_media(State(db), Extension(auth_user), Query(params)).await
}

/// 根据ID获取单个媒体项目
pub async fn get_media_by_id(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Path(media_id): Path<String>,
) -> Result<Json<MediaItem>, StatusCode> {
    let query = "SELECT * FROM media_files WHERE id = $1 AND user_id = $2 AND status = 'active'";
    
    match sqlx::query_as::<_, MediaItem>(query)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .fetch_one(&db.pool)
        .await
    {
        Ok(media) => Ok(Json(media)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error getting media by id: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 更新媒体项目
pub async fn update_media(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Path(media_id): Path<String>,
    AxumJson(payload): AxumJson<UpdateMediaRequest>,
) -> Result<Json<MediaItem>, StatusCode> {
    let now = Utc::now();
    
    let query = r#"
        UPDATE media_files 
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            updated_at = $3
        WHERE id = $4 AND user_id = $5 AND status = 'active'
        RETURNING *
    "#;
    
    match sqlx::query_as::<_, MediaItem>(query)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(&now)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .fetch_one(&db.pool)
        .await
    {
        Ok(media) => Ok(Json(media)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error updating media: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 删除媒体项目
pub async fn delete_media(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Path(media_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let now = Utc::now();
    
    let query = r#"
        UPDATE media_files 
        SET status = 'deleted', updated_at = $1
        WHERE id = $2 AND user_id = $3 AND status = 'active'
    "#;
    
    match sqlx::query(query)
        .bind(&now)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .execute(&db.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            eprintln!("Database error deleting media: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UploadMediaRequest {
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub content_type: String,
    pub cos_key: String,
    pub cos_url: String,
    pub cos_bucket: String,
    pub cos_region: String,
    pub media_type: String,
}

/// 上传媒体文件后更新记录
pub async fn upload_media_file(
    State(db): State<Database>,
    Extension(auth_user): Extension<AuthUser>,
    Path(media_id): Path<String>,
    AxumJson(payload): AxumJson<UploadMediaRequest>,
) -> Result<Json<MediaItem>, StatusCode> {
    let now = Utc::now();
    
    let query = r#"
        UPDATE media_files 
        SET filename = $1,
            original_filename = $2,
            file_size = $3,
            content_type = $4,
            cos_key = $5,
            cos_url = $6,
            cos_bucket = $7,
            cos_region = $8,
            media_type = $9,
            updated_at = $10
        WHERE id = $11 AND user_id = $12 AND status = 'active'
        RETURNING *
    "#;
    
    match sqlx::query_as::<_, MediaItem>(query)
        .bind(&payload.filename)
        .bind(&payload.original_filename)
        .bind(&payload.file_size)
        .bind(&payload.content_type)
        .bind(&payload.cos_key)
        .bind(&payload.cos_url)
        .bind(&payload.cos_bucket)
        .bind(&payload.cos_region)
        .bind(&payload.media_type)
        .bind(&now)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .fetch_one(&db.pool)
        .await
    {
        Ok(media) => Ok(Json(media)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error uploading media file: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}