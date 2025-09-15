use sqlx::{Pool, Postgres, PgPool};
use std::env;
use tracing::{info, error};

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/media_hub".to_string());
        
        info!("连接数据库: {}", database_url);
        
        let pool = PgPool::connect(&database_url).await?;
        
        // 运行迁移
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| {
                error!("数据库迁移失败: {}", e);
                e
            })?;
        
        info!("数据库连接和迁移成功完成");
        
        Ok(Database { pool })
    }
    
    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}