use crate::credentials::AuthUser;
use crate::database::Database;
use crate::handlers::cos_handlers;
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    let mut query =
        "SELECT * FROM media_files WHERE user_id = $1 AND status = 'active'".to_string();
    let mut query_params: Vec<String> = vec![auth_user.user_id.clone()];
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
        let search_param = format!("%{}%", search);
        query.push_str(&format!(
            " AND (title ILIKE ${} OR description ILIKE ${})",
            param_count, param_count
        ));
        query_params.push(search_param);
    }

    query.push_str(" ORDER BY created_at DESC");

    // 获取总数 - 构建相同的查询条件
    let count_query = query
        .replace("SELECT *", "SELECT COUNT(*)")
        .replace(" ORDER BY created_at DESC", "");

    // 为总数查询绑定所有参数
    let mut count_query_builder = sqlx::query_scalar(&count_query);
    for param in &query_params {
        count_query_builder = count_query_builder.bind(param);
    }

    let total: i64 = match count_query_builder.fetch_one(&db.pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Database error getting media count: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // 添加分页
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    // 为主查询绑定所有参数
    let mut main_query_builder = sqlx::query_as::<_, MediaItem>(&query);
    for param in &query_params {
        main_query_builder = main_query_builder.bind(param);
    }

    let rows = match main_query_builder.fetch_all(&db.pool).await {
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
    println!(
        "🚀 收到创建媒体请求 - 用户: {}, 标题: {}",
        auth_user.user_id, payload.title
    );
    println!("📋 媒体数据: {:?}", payload);

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

    println!("💾 准备插入数据库 - 媒体ID: {}", media_id);

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
        Ok(result) => {
            println!(
                "✅ 媒体记录创建成功 - ID: {}, 影响行数: {}",
                media_id,
                result.rows_affected()
            );
            Ok(Json(media_item))
        }
        Err(e) => {
            eprintln!("❌ 数据库错误 - 创建媒体失败: {}", e);
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
    // 首先获取媒体项目信息，用于删除COS文件
    let get_query =
        "SELECT * FROM media_files WHERE id = $1 AND user_id = $2 AND status = 'active'";

    let media_item = match sqlx::query_as::<_, MediaItem>(get_query)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .fetch_one(&db.pool)
        .await
    {
        Ok(item) => item,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error getting media for deletion: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // 从腾讯云COS删除文件
    if let Err(e) = cos_handlers::delete_cos_file(
        &media_item.cos_key,
        &media_item.cos_bucket,
        &media_item.cos_region,
    )
    .await
    {
        eprintln!("Failed to delete file from COS: {}", e);
        // 注意：即使COS删除失败，我们仍然继续删除数据库记录
        // 这样可以避免数据库中留下无效的记录
        crate::log_with_storage!(warn, "COS文件删除失败，但继续删除数据库记录: {}", e);
    } else {
        crate::log_with_storage!(info, "成功从COS删除文件: {}", media_item.cos_key);
    }

    // 从数据库硬删除记录
    let delete_query = "DELETE FROM media_files WHERE id = $1 AND user_id = $2";

    match sqlx::query(delete_query)
        .bind(&media_id)
        .bind(&auth_user.user_id)
        .execute(&db.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                crate::log_with_storage!(info, "成功删除媒体项目: {}", media_id);
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
