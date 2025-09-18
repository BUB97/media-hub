-- 创建AI分析结果表
CREATE TABLE IF NOT EXISTS ai_analysis (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    analysis_type TEXT NOT NULL, -- 'ImageDescription', 'VideoSummary', 'DocumentExtraction', 'ContentTagging', 'SimilaritySearch'
    result_data TEXT NOT NULL,   -- JSON格式的分析结果
    confidence_score REAL,       -- 置信度分数 (0.0-1.0)
    processing_time_ms BIGINT,   -- 处理时间（毫秒）
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    
    -- 外键约束
    CONSTRAINT fk_ai_analysis_media FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_ai_analysis_media_id ON ai_analysis(media_id);
CREATE INDEX IF NOT EXISTS idx_ai_analysis_type ON ai_analysis(analysis_type);
CREATE INDEX IF NOT EXISTS idx_ai_analysis_created_at ON ai_analysis(created_at);
CREATE INDEX IF NOT EXISTS idx_ai_analysis_confidence ON ai_analysis(confidence_score);

-- 创建复合索引用于媒体分析查询
CREATE INDEX IF NOT EXISTS idx_ai_analysis_media_type ON ai_analysis(media_id, analysis_type);

-- 创建向量嵌入表（用于相似度搜索）
CREATE TABLE IF NOT EXISTS ai_embeddings (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    embedding_type TEXT NOT NULL, -- 'image', 'text', 'multimodal'
    embedding_vector REAL[] NOT NULL, -- 向量数据
    model_name TEXT NOT NULL,     -- 使用的模型名称
    created_at TIMESTAMPTZ NOT NULL,
    
    -- 外键约束
    CONSTRAINT fk_ai_embeddings_media FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

-- 创建向量索引（如果支持的话）
CREATE INDEX IF NOT EXISTS idx_ai_embeddings_media_id ON ai_embeddings(media_id);
CREATE INDEX IF NOT EXISTS idx_ai_embeddings_type ON ai_embeddings(embedding_type);
CREATE INDEX IF NOT EXISTS idx_ai_embeddings_model ON ai_embeddings(model_name);

-- 创建批量分析任务表
CREATE TABLE IF NOT EXISTS ai_batch_jobs (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    job_name TEXT,
    total_items INTEGER NOT NULL DEFAULT 0,
    completed_items INTEGER NOT NULL DEFAULT 0,
    failed_items INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed', 'cancelled'
    job_config TEXT NOT NULL,              -- JSON格式的任务配置
    error_message TEXT,                    -- 错误信息
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    
    -- 外键约束
    CONSTRAINT fk_ai_batch_jobs_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 创建批量任务索引
CREATE INDEX IF NOT EXISTS idx_ai_batch_jobs_user_id ON ai_batch_jobs(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_batch_jobs_status ON ai_batch_jobs(status);
CREATE INDEX IF NOT EXISTS idx_ai_batch_jobs_created_at ON ai_batch_jobs(created_at);

-- 创建AI标签表（用于内容标签管理）
CREATE TABLE IF NOT EXISTS ai_tags (
    id TEXT PRIMARY KEY NOT NULL,
    tag_name TEXT NOT NULL UNIQUE,
    category TEXT,                        -- 标签分类
    description TEXT,                     -- 标签描述
    usage_count INTEGER NOT NULL DEFAULT 0, -- 使用次数
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- 创建标签索引
CREATE INDEX IF NOT EXISTS idx_ai_tags_name ON ai_tags(tag_name);
CREATE INDEX IF NOT EXISTS idx_ai_tags_category ON ai_tags(category);
CREATE INDEX IF NOT EXISTS idx_ai_tags_usage_count ON ai_tags(usage_count DESC);

-- 创建媒体标签关联表
CREATE TABLE IF NOT EXISTS ai_media_tags (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    confidence REAL NOT NULL DEFAULT 1.0, -- 标签置信度
    source TEXT NOT NULL,                 -- 标签来源: 'ai', 'user', 'system'
    created_at TIMESTAMPTZ NOT NULL,
    
    -- 外键约束
    CONSTRAINT fk_ai_media_tags_media FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE,
    CONSTRAINT fk_ai_media_tags_tag FOREIGN KEY (tag_id) REFERENCES ai_tags(id) ON DELETE CASCADE,
    
    -- 唯一约束：同一媒体文件的同一标签只能有一个记录
    CONSTRAINT uk_ai_media_tags_media_tag UNIQUE (media_id, tag_id)
);

-- 创建媒体标签关联索引
CREATE INDEX IF NOT EXISTS idx_ai_media_tags_media_id ON ai_media_tags(media_id);
CREATE INDEX IF NOT EXISTS idx_ai_media_tags_tag_id ON ai_media_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_ai_media_tags_confidence ON ai_media_tags(confidence DESC);
CREATE INDEX IF NOT EXISTS idx_ai_media_tags_source ON ai_media_tags(source);