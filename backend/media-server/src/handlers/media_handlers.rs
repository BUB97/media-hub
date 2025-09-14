use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MediaItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub media_type: String,
}

#[derive(Deserialize)]
pub struct CreateMediaRequest {
    pub title: String,
    pub description: String,
    pub media_type: String,
}

// 获取所有媒体项目
pub async fn get_media() -> Json<Vec<MediaItem>> {
    crate::log_with_storage!(info, "开始获取所有媒体项目");
    
    // 这里返回模拟数据，实际项目中会从数据库获取
    let media_items = vec![
        MediaItem {
            id: 1,
            title: "Sample Video".to_string(),
            description: "A sample video file".to_string(),
            media_type: "video".to_string(),
        },
        MediaItem {
            id: 2,
            title: "Sample Audio".to_string(),
            description: "A sample audio file".to_string(),
            media_type: "audio".to_string(),
        },
    ];
    
    crate::log_with_storage!(debug, "返回 {} 个媒体项目", media_items.len());
    Json(media_items)
}

// 创建新的媒体项目
pub async fn create_media(Json(payload): Json<CreateMediaRequest>) -> (StatusCode, Json<MediaItem>) {
    crate::log_with_storage!(info, "创建新媒体项目: {}", payload.title);
    
    // 这里是模拟创建，实际项目中会保存到数据库
    let new_media = MediaItem {
        id: 3, // 实际中应该是自动生成的ID
        title: payload.title.clone(),
        description: payload.description,
        media_type: payload.media_type,
    };
    
    crate::log_with_storage!(debug, "成功创建媒体项目，ID: {}", new_media.id);
    (StatusCode::CREATED, Json(new_media))
}

// 根据查询参数搜索媒体
pub async fn search_media(Query(params): Query<HashMap<String, String>>) -> Json<Vec<MediaItem>> {
    let search_term = params.get("q").unwrap_or(&String::new()).to_lowercase();
    
    // 模拟搜索逻辑
    let all_media = vec![
        MediaItem {
            id: 1,
            title: "Sample Video".to_string(),
            description: "A sample video file".to_string(),
            media_type: "video".to_string(),
        },
        MediaItem {
            id: 2,
            title: "Sample Audio".to_string(),
            description: "A sample audio file".to_string(),
            media_type: "audio".to_string(),
        },
    ];
    
    let filtered_media: Vec<MediaItem> = all_media
        .into_iter()
        .filter(|item| {
            item.title.to_lowercase().contains(&search_term)
                || item.description.to_lowercase().contains(&search_term)
        })
        .collect();
    
    Json(filtered_media)
}