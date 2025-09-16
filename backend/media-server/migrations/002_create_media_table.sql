-- 创建媒体文件表
CREATE TABLE IF NOT EXISTS media_files (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    filename TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    content_type TEXT NOT NULL,
    cos_key TEXT NOT NULL,
    cos_url TEXT NOT NULL,
    cos_bucket TEXT NOT NULL,
    cos_region TEXT NOT NULL,
    media_type TEXT NOT NULL, -- 'image', 'video', 'audio', 'document'
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'deleted', 'processing'
    metadata JSONB, -- 存储额外的元数据（如分辨率、时长等）
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,
    
    -- 外键约束
    CONSTRAINT fk_media_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_media_user_id ON media_files(user_id);
CREATE INDEX IF NOT EXISTS idx_media_type ON media_files(media_type);
CREATE INDEX IF NOT EXISTS idx_media_status ON media_files(status);
CREATE INDEX IF NOT EXISTS idx_media_created_at ON media_files(created_at);
CREATE INDEX IF NOT EXISTS idx_media_cos_key ON media_files(cos_key);

-- 创建复合索引用于用户媒体查询
CREATE INDEX IF NOT EXISTS idx_media_user_status_created ON media_files(user_id, status, created_at DESC);