use axum::{
    extract::{State, Path, Extension},
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::database::Database;
use crate::credentials::AuthUser;
use crate::ai_client::{AiClient, AiAnalysisRequest, AiSimilarityRequest, AiEmbeddingRequest};

/// AI 分析类型枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisType {
    ImageDescription,    // 图像描述
    VideoSummary,       // 视频总结
    DocumentExtraction, // 文档提取
    ContentTagging,     // 内容标签
    SimilaritySearch,   // 相似度搜索
}

/// AI 分析请求
#[derive(Deserialize, Debug)]
pub struct AnalysisRequest {
    pub media_id: String,
    pub analysis_type: AnalysisType,
    pub options: Option<AnalysisOptions>,
}

/// 分析选项
#[derive(Deserialize, Debug)]
pub struct AnalysisOptions {
    pub language: Option<String>,           // 分析语言
    pub detail_level: Option<String>,       // 详细程度: basic, detailed, comprehensive
    pub include_objects: Option<bool>,      // 是否包含对象检测
    pub include_text: Option<bool>,         // 是否包含文本提取
    pub include_sentiment: Option<bool>,    // 是否包含情感分析
    pub max_tags: Option<u32>,             // 最大标签数量
}

/// AI 分析结果
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct AnalysisResult {
    pub id: String,
    pub media_id: String,
    pub analysis_type: String,
    pub result_data: String,               // JSON 格式的分析结果
    pub confidence_score: Option<f32>,     // 置信度分数
    pub processing_time_ms: Option<i64>,   // 处理时间（毫秒）
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 分析结果详情
#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResultDetail {
    pub description: Option<String>,        // 内容描述
    pub objects: Option<Vec<DetectedObject>>, // 检测到的对象
    pub tags: Option<Vec<ContentTag>>,      // 内容标签
    pub text_content: Option<String>,       // 提取的文本
    pub sentiment: Option<SentimentAnalysis>, // 情感分析
    pub summary: Option<String>,            // 内容总结
    pub key_points: Option<Vec<String>>,    // 关键点
}

/// 检测到的对象
#[derive(Serialize, Deserialize, Debug)]
pub struct DetectedObject {
    pub name: String,
    pub confidence: f32,
    pub bounding_box: Option<BoundingBox>,
}

/// 边界框
#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 内容标签
#[derive(Serialize, Deserialize, Debug)]
pub struct ContentTag {
    pub tag: String,
    pub confidence: f32,
    pub category: Option<String>,
}

/// 情感分析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct SentimentAnalysis {
    pub sentiment: String,      // positive, negative, neutral
    pub confidence: f32,
    pub emotions: Option<HashMap<String, f32>>, // 具体情感分数
}

/// 相似度搜索请求
#[derive(Deserialize, Debug)]
pub struct SimilaritySearchRequest {
    pub query: String,                    // 查询文本
    pub media_types: Option<Vec<String>>, // 媒体类型过滤
    pub limit: Option<u32>,               // 结果数量限制
    pub threshold: Option<f32>,           // 相似度阈值
}

/// 相似度搜索结果
#[derive(Serialize, Debug)]
pub struct SimilaritySearchResult {
    pub media_id: String,
    pub similarity_score: f32,
    pub matched_content: String,
    pub media_info: MediaInfo,
}

/// 媒体信息
#[derive(Serialize, Debug)]
pub struct MediaInfo {
    pub title: String,
    pub media_type: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
}

/// 批量分析请求
#[derive(Deserialize, Debug)]
pub struct BatchAnalysisRequest {
    pub media_ids: Vec<String>,
    pub analysis_types: Vec<AnalysisType>,
    pub options: Option<AnalysisOptions>,
}

/// 批量分析状态
#[derive(Serialize, Debug)]
pub struct BatchAnalysisStatus {
    pub batch_id: String,
    pub total_items: u32,
    pub completed_items: u32,
    pub failed_items: u32,
    pub status: String,           // pending, processing, completed, failed
    pub created_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

/// 分析统计信息
#[derive(Serialize, Debug)]
pub struct AnalysisStats {
    pub total_analyses: u64,
    pub analyses_by_type: HashMap<String, u64>,
    pub average_processing_time: f64,
    pub success_rate: f32,
    pub most_common_tags: Vec<(String, u64)>,
}

// ============= API 处理函数 =============

/// 创建 AI 分析任务
pub async fn create_analysis(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
    AxumJson(request): AxumJson<AnalysisRequest>,
) -> Result<Json<AnalysisResult>, StatusCode> {
    crate::log_with_storage!(info, "用户 {} 请求分析媒体 {}", user.user_id, request.media_id);

    // 验证媒体是否存在且属于当前用户
    let media_exists = sqlx::query!(
        "SELECT id, cos_url, media_type FROM media_files WHERE id = $1 AND user_id = $2",
        request.media_id,
        user.user_id
    )
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let media_info = match media_exists {
        Some(info) => info,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // 检查是否已有相同类型的分析结果
    let existing_analysis = sqlx::query_as!(
        AnalysisResult,
        "SELECT * FROM ai_analysis WHERE media_id = $1 AND analysis_type = $2",
        request.media_id,
        format!("{:?}", request.analysis_type)
    )
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(existing) = existing_analysis {
        crate::log_with_storage!(info, "返回已存在的分析结果: {}", existing.id);
        return Ok(Json(existing));
    }

    // 创建新的分析任务
    let analysis_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // 构建媒体文件URL（使用COS URL）
    let media_url = media_info.cos_url;

    // 准备AI分析选项
    let mut ai_options = HashMap::new();
    if let Some(options) = &request.options {
        if let Some(detail_level) = &options.detail_level {
            ai_options.insert("detail_level".to_string(), serde_json::Value::String(detail_level.clone()));
        }
        if let Some(include_objects) = options.include_objects {
            ai_options.insert("include_objects".to_string(), serde_json::Value::Bool(include_objects));
        }
        if let Some(include_text) = options.include_text {
            ai_options.insert("include_text".to_string(), serde_json::Value::Bool(include_text));
        }
        if let Some(include_sentiment) = options.include_sentiment {
            ai_options.insert("include_sentiment".to_string(), serde_json::Value::Bool(include_sentiment));
        }
    }

    // 创建AI客户端并发送分析请求
    let ai_client = AiClient::default();
    let ai_request = AiAnalysisRequest {
        analysis_id: analysis_id.clone(),
        media_id: request.media_id.clone(),
        media_url,
        analysis_type: match request.analysis_type {
            AnalysisType::ImageDescription => "image_description".to_string(),
            AnalysisType::VideoSummary => "video_summary".to_string(),
            AnalysisType::DocumentExtraction => "text_extraction".to_string(),
            AnalysisType::ContentTagging => "object_detection".to_string(),
            AnalysisType::SimilaritySearch => "scene_analysis".to_string(),
        },
        options: if ai_options.is_empty() { None } else { Some(ai_options) },
        user_id: Some(user.user_id.clone()),
    };

    // 先保存初始状态到数据库
    let placeholder_result = AnalysisResultDetail {
        description: Some("AI 分析正在处理中...".to_string()),
        objects: None,
        tags: None,
        text_content: None,
        sentiment: None,
        summary: None,
        key_points: None,
    };

    let result_json = serde_json::to_string(&placeholder_result)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut analysis_result = AnalysisResult {
        id: analysis_id.clone(),
        media_id: request.media_id.clone(),
        analysis_type: format!("{:?}", request.analysis_type),
        result_data: result_json,
        confidence_score: None,
        processing_time_ms: None,
        created_at: now,
        updated_at: now,
    };

    // 保存初始状态到数据库
    sqlx::query!(
        r#"
        INSERT INTO ai_analysis (id, media_id, analysis_type, result_data, confidence_score, processing_time_ms, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        analysis_result.id,
        analysis_result.media_id,
        analysis_result.analysis_type,
        analysis_result.result_data,
        analysis_result.confidence_score,
        analysis_result.processing_time_ms,
        analysis_result.created_at,
        analysis_result.updated_at
    )
    .execute(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to save analysis result: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 异步调用AI服务进行分析
    let db_clone = db.clone();
    let analysis_id_clone = analysis_id.clone();
    tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        
        match ai_client.analyze_image(ai_request).await {
            Ok(ai_response) => {
                let processing_time = start_time.elapsed().as_millis() as i64;
                
                // 解析AI响应并构建详细结果
                let mut detailed_result = AnalysisResultDetail {
                    description: None,
                    objects: None,
                    tags: None,
                    text_content: None,
                    sentiment: None,
                    summary: None,
                    key_points: None,
                };

                // 从AI响应中提取结构化数据
                if let Some(content) = ai_response.result.get("content") {
                    if let Some(content_str) = content.as_str() {
                        detailed_result.description = Some(content_str.to_string());
                    }
                }

                if let Some(structured_data) = ai_response.result.get("structured_data") {
                    if let Some(objects_data) = structured_data.get("objects") {
                        if let Ok(objects) = serde_json::from_value::<Vec<DetectedObject>>(objects_data.clone()) {
                            detailed_result.objects = Some(objects);
                        }
                    }
                    
                    if let Some(text_data) = structured_data.get("extracted_text") {
                        if let Some(text_array) = text_data.as_array() {
                            if let Some(first_text) = text_array.first() {
                                if let Some(text_str) = first_text.as_str() {
                                    detailed_result.text_content = Some(text_str.to_string());
                                }
                            }
                        }
                    }
                }

                let updated_result_json = match serde_json::to_string(&detailed_result) {
                    Ok(json) => json,
                    Err(_) => ai_response.result.get("content")
                        .and_then(|v| v.as_str())
                        .unwrap_or("AI分析完成")
                        .to_string(),
                };

                // 更新数据库中的分析结果
                if let Err(e) = sqlx::query!(
                    "UPDATE ai_analysis SET result_data = $1, processing_time_ms = $2, updated_at = $3 WHERE id = $4",
                    updated_result_json,
                    processing_time,
                    Utc::now(),
                    analysis_id_clone
                )
                .execute(&db_clone.pool)
                .await
                {
                    eprintln!("Failed to update analysis result: {}", e);
                }

                crate::log_with_storage!(info, "AI分析完成: {} ({}ms)", analysis_id_clone, processing_time);
            }
            Err(e) => {
                eprintln!("AI analysis failed: {}", e);
                
                // 更新为错误状态
                let error_result = AnalysisResultDetail {
                    description: Some(format!("AI分析失败: {}", e)),
                    objects: None,
                    tags: None,
                    text_content: None,
                    sentiment: None,
                    summary: None,
                    key_points: None,
                };

                if let Ok(error_json) = serde_json::to_string(&error_result) {
                    let _ = sqlx::query!(
                        "UPDATE ai_analysis SET result_data = $1, updated_at = $2 WHERE id = $3",
                        error_json,
                        Utc::now(),
                        analysis_id_clone
                    )
                    .execute(&db_clone.pool)
                    .await;
                }

                crate::log_with_storage!(error, "AI分析失败: {}", analysis_id_clone);
            }
        }
    });

    crate::log_with_storage!(info, "创建AI分析任务成功: {}", analysis_id);
    Ok(Json(analysis_result))
}

/// 获取分析结果
pub async fn get_analysis(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
    Path(analysis_id): Path<String>,
) -> Result<Json<AnalysisResult>, StatusCode> {
    let analysis = sqlx::query_as!(
        AnalysisResult,
        r#"
        SELECT a.* FROM ai_analysis a
        JOIN media_files m ON a.media_id = m.id
        WHERE a.id = $1 AND m.user_id = $2
        "#,
        analysis_id,
        user.user_id
    )
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match analysis {
        Some(result) => Ok(Json(result)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 获取媒体的所有分析结果
pub async fn get_media_analyses(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
    Path(media_id): Path<String>,
) -> Result<Json<Vec<AnalysisResult>>, StatusCode> {
    let analyses = sqlx::query_as!(
        AnalysisResult,
        r#"
        SELECT a.* FROM ai_analysis a
        JOIN media_files m ON a.media_id = m.id
        WHERE a.media_id = $1 AND m.user_id = $2
        ORDER BY a.created_at DESC
        "#,
        media_id,
        user.user_id
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(analyses))
}

/// 相似度搜索
pub async fn similarity_search(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
    AxumJson(request): AxumJson<SimilaritySearchRequest>,
) -> Result<Json<Vec<SimilaritySearchResult>>, StatusCode> {
    crate::log_with_storage!(info, "用户 {} 执行相似度搜索: {}", user.user_id, request.query);

    // 创建AI客户端并发送相似度搜索请求
    let ai_client = AiClient::default();
    let ai_request = AiSimilarityRequest {
        query: request.query.clone(),
        limit: request.limit.unwrap_or(10) as i32,
        threshold: request.threshold.unwrap_or(0.7),
        user_id: Some(user.user_id.clone()),
    };

    match ai_client.similarity_search(ai_request).await {
        Ok(ai_response) => {
            let mut results = Vec::new();
            
            // 解析AI响应中的相似度搜索结果
            for ai_result in ai_response.results {
                // 从数据库获取媒体信息
                if let Ok(Some(media_info)) = sqlx::query!(
                    "SELECT title, media_type, file_size, created_at FROM media_files WHERE id = $1 AND user_id = $2",
                    ai_result.media_id,
                    user.user_id
                )
                .fetch_optional(&db.pool)
                .await
                {
                    let result = SimilaritySearchResult {
                        media_id: ai_result.media_id,
                        similarity_score: ai_result.similarity_score,
                        matched_content: ai_result.content,
                        media_info: MediaInfo {
                            title: media_info.title,
                            media_type: media_info.media_type,
                            file_size: media_info.file_size,
                            created_at: media_info.created_at,
                        },
                    };
                    results.push(result);
                }
            }

            crate::log_with_storage!(info, "相似度搜索完成，找到 {} 个结果", results.len());
            Ok(Json(results))
        }
        Err(e) => {
            eprintln!("Similarity search failed: {}", e);
            crate::log_with_storage!(error, "相似度搜索失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取分析统计信息
pub async fn get_analysis_stats(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<AnalysisStats>, StatusCode> {
    // 获取总分析数量
    let total_count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM ai_analysis a
        JOIN media_files m ON a.media_id = m.id
        WHERE m.user_id = $1
        "#,
        user.user_id
    )
    .fetch_one(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 按类型统计
    let type_stats = sqlx::query!(
        r#"
        SELECT analysis_type, COUNT(*) as count FROM ai_analysis a
        JOIN media_files m ON a.media_id = m.id
        WHERE m.user_id = $1
        GROUP BY analysis_type
        "#,
        user.user_id
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut analyses_by_type = HashMap::new();
    for stat in type_stats {
        analyses_by_type.insert(stat.analysis_type, stat.count.unwrap_or(0) as u64);
    }

    let stats = AnalysisStats {
        total_analyses: total_count.count.unwrap_or(0) as u64,
        analyses_by_type,
        average_processing_time: 0.0, // TODO: 计算平均处理时间
        success_rate: 1.0,             // TODO: 计算成功率
        most_common_tags: vec![],      // TODO: 统计最常见标签
    };

    Ok(Json(stats))
}

/// 删除分析结果
pub async fn delete_analysis(
    State(db): State<Database>,
    Extension(user): Extension<AuthUser>,
    Path(analysis_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        r#"
        DELETE FROM ai_analysis 
        WHERE id = $1 AND media_id IN (
            SELECT id FROM media_files WHERE user_id = $2
        )
        "#,
        analysis_id,
        user.user_id
    )
    .execute(&db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() > 0 {
        crate::log_with_storage!(info, "删除AI分析结果: {}", analysis_id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}