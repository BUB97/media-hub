use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info, warn};

/// AI服务客户端配置
#[derive(Debug, Clone)]
pub struct AiClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl Default for AiClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8001".to_string(),
            timeout: Duration::from_secs(60),
            max_retries: 3,
        }
    }
}

/// AI服务客户端
#[derive(Debug, Clone)]
pub struct AiClient {
    client: Client,
    config: AiClientConfig,
}

/// AI分析请求
#[derive(Serialize, Debug)]
pub struct AiAnalysisRequest {
    pub analysis_id: String,
    pub media_id: String,
    pub media_url: String,
    pub analysis_type: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
    pub user_id: Option<String>,
}

/// AI分析响应
#[derive(Deserialize, Debug)]
pub struct AiAnalysisResponse {
    pub analysis_id: String,
    pub media_id: String,
    pub result: HashMap<String, serde_json::Value>,
    pub success: bool,
    pub error: Option<String>,
    pub timestamp: String,
}

/// 相似度搜索请求
#[derive(Serialize, Debug)]
pub struct AiSimilarityRequest {
    pub query: String,
    pub limit: i32,
    pub threshold: f32,
    pub user_id: Option<String>,
}

/// 相似度搜索结果
#[derive(Deserialize, Debug)]
pub struct AiSimilarityResult {
    pub media_id: String,
    pub content: String,
    pub similarity_score: f32,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 相似度搜索响应
#[derive(Deserialize, Debug)]
pub struct AiSimilarityResponse {
    pub query: String,
    pub results: Vec<AiSimilarityResult>,
    pub success: bool,
    pub error: Option<String>,
    pub timestamp: String,
}

/// 嵌入存储请求
#[derive(Serialize, Debug)]
pub struct AiEmbeddingRequest {
    pub media_id: String,
    pub content: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub user_id: Option<String>,
}

/// 嵌入存储响应
#[derive(Deserialize, Debug)]
pub struct AiEmbeddingResponse {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl AiClient {
    /// 创建新的AI客户端
    pub fn new(config: AiClientConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// 使用默认配置创建AI客户端
    pub fn default() -> Self {
        Self::new(AiClientConfig::default())
    }

    /// 检查AI服务健康状态
    pub async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/health", self.config.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("AI service health check passed");
                    Ok(true)
                } else {
                    warn!("AI service health check failed with status: {}", response.status());
                    Ok(false)
                }
            }
            Err(e) => {
                error!("AI service health check error: {}", e);
                Err(Box::new(e))
            }
        }
    }

    /// 发送图像分析请求
    pub async fn analyze_image(
        &self,
        request: AiAnalysisRequest,
    ) -> Result<AiAnalysisResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/analyze/image", self.config.base_url);
        
        for attempt in 1..=self.config.max_retries {
            match self.client.post(&url).json(&request).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<AiAnalysisResponse>().await {
                            Ok(result) => {
                                info!("Image analysis completed for media_id: {}", request.media_id);
                                return Ok(result);
                            }
                            Err(e) => {
                                error!("Failed to parse AI response: {}", e);
                                return Err(Box::new(e));
                            }
                        }
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        error!("AI service returned error {}: {}", status, error_text);
                        
                        if attempt == self.config.max_retries {
                            return Err(format!("AI service error after {} attempts: {}", self.config.max_retries, error_text).into());
                        }
                    }
                }
                Err(e) => {
                    error!("HTTP request failed (attempt {}): {}", attempt, e);
                    
                    if attempt == self.config.max_retries {
                        return Err(Box::new(e));
                    }
                }
            }
            
            // 重试前等待
            tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
        }
        
        Err("Max retries exceeded".into())
    }

    /// 执行相似度搜索
    pub async fn similarity_search(
        &self,
        request: AiSimilarityRequest,
    ) -> Result<AiSimilarityResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/search/similarity", self.config.base_url);
        
        match self.client.post(&url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<AiSimilarityResponse>().await {
                        Ok(result) => {
                            info!("Similarity search completed for query: {}", request.query);
                            Ok(result)
                        }
                        Err(e) => {
                            error!("Failed to parse similarity search response: {}", e);
                            Err(Box::new(e))
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    error!("Similarity search failed with status {}: {}", status, error_text);
                    Err(format!("Similarity search failed: {}", error_text).into())
                }
            }
            Err(e) => {
                error!("Similarity search HTTP request failed: {}", e);
                Err(Box::new(e))
            }
        }
    }

    /// 存储内容嵌入
    pub async fn store_embedding(
        &self,
        request: AiEmbeddingRequest,
    ) -> Result<AiEmbeddingResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/embeddings/store", self.config.base_url);
        
        match self.client.post(&url).json(&request).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<AiEmbeddingResponse>().await {
                        Ok(result) => {
                            info!("Embedding stored for media_id: {}", request.media_id);
                            Ok(result)
                        }
                        Err(e) => {
                            error!("Failed to parse embedding response: {}", e);
                            Err(Box::new(e))
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    error!("Store embedding failed with status {}: {}", status, error_text);
                    Err(format!("Store embedding failed: {}", error_text).into())
                }
            }
            Err(e) => {
                error!("Store embedding HTTP request failed: {}", e);
                Err(Box::new(e))
            }
        }
    }

    /// 获取AI服务统计信息
    pub async fn get_service_stats(&self) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/stats", self.config.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HashMap<String, serde_json::Value>>().await {
                        Ok(stats) => {
                            info!("Retrieved AI service stats");
                            Ok(stats)
                        }
                        Err(e) => {
                            error!("Failed to parse service stats: {}", e);
                            Err(Box::new(e))
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    warn!("Get service stats failed with status {}: {}", status, error_text);
                    Ok(HashMap::new()) // 返回空统计信息而不是错误
                }
            }
            Err(e) => {
                warn!("Service stats HTTP request failed: {}", e);
                Ok(HashMap::new()) // 返回空统计信息而不是错误
            }
        }
    }
}