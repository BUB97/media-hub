# 日志、错误监控和查询功能整合指南

## 🎯 整合概述

本项目已成功整合了完整的日志、错误监控和查询功能，提供了一套完整的可观测性解决方案。

## 📋 功能特性

### 1. 多层次日志记录
- **控制台输出**: 开发时实时查看日志
- **文件持久化**: JSON 格式日志文件，按天轮转
- **内存存储**: 最近 1000 条日志的快速查询
- **结构化日志**: 包含时间戳、级别、消息、用户ID、请求ID等字段

### 2. 错误监控
- **实时计数**: 错误和警告数量统计
- **监控端点**: `/api/metrics` 提供系统运行指标
- **告警机制**: 可扩展的错误阈值监控

### 3. 日志查询系统
- **多维度查询**: 支持按用户ID、日志级别、时间范围查询
- **分页支持**: 限制返回数量，避免大量数据传输
- **实时查询**: `/api/logs` 端点提供灵活的查询接口

## 🏗️ 系统架构

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   应用层        │    │    日志层        │    │   存储层        │
│                 │    │                  │    │                 │
│ • HTTP 端点     │───▶│ • tracing 宏     │───▶│ • 文件存储      │
│ • 业务逻辑      │    │ • 日志过滤       │    │ • 内存缓存      │
│ • 错误处理      │    │ • 格式化输出     │    │ • 日志轮转      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌──────────────────┐
                       │    查询层        │
                       │                  │
                       │ • REST API       │
                       │ • 条件过滤       │
                       │ • 结果分页       │
                       └──────────────────┘
```

## 🔧 核心组件

### 1. 日志模块 (`src/logging.rs`)
```rust
// 全局日志存储
pub static LOG_STORAGE: RwLock<Vec<LogEntry>> = RwLock::const_new(Vec::new());
pub static ERROR_COUNT: AtomicU64 = AtomicU64::new(0);
pub static WARN_COUNT: AtomicU64 = AtomicU64::new(0);

// 日志条目结构
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub module: String,
    pub fields: HashMap<String, String>,
}
```

### 2. 日志宏系统
```rust
// 基础日志宏
log_with_storage!(info, "操作完成");

// 带用户信息的日志宏
log_with_user!(info, user_id, request_id, "用户操作: {}", action);
```

### 3. API 端点

#### 日志查询端点
```http
GET /api/logs?user_id=123&level=error&start_time=2025-01-01T00:00:00Z&limit=50
```

#### 监控指标端点
```http
GET /api/metrics
```

## 📊 使用示例

### 1. 基础日志记录
```rust
#[instrument]
async fn create_media(Json(payload): Json<CreateMediaRequest>) -> Json<MediaItem> {
    info!("创建媒体请求: {}", payload.title);
    
    // 业务逻辑
    let result = process_media(payload).await;
    
    match result {
        Ok(media) => {
            info!("媒体创建成功，ID: {}", media.id);
            Json(media)
        }
        Err(e) => {
            error!("媒体创建失败: {}", e);
            // 错误处理
        }
    }
}
```

### 2. 查询日志记录
```bash
# 查询所有日志
curl "http://localhost:3000/api/logs"

# 按用户查询
curl "http://localhost:3000/api/logs?user_id=user123"

# 按级别查询
curl "http://localhost:3000/api/logs?level=error"

# 按时间范围查询
curl "http://localhost:3000/api/logs?start_time=2025-01-01T00:00:00Z&end_time=2025-01-02T00:00:00Z"
```

### 3. 监控系统状态
```bash
# 获取系统指标
curl "http://localhost:3000/api/metrics"

# 响应示例
{
  "error_count": 5,
  "warn_count": 12,
  "uptime_seconds": 3600,
  "total_requests": 1250
}
```

## 🔍 日志级别使用指南

### 生产环境建议
- **ERROR**: 系统错误、业务异常
- **WARN**: 性能警告、配置问题
- **INFO**: 关键业务操作、用户行为
- **DEBUG**: 详细执行流程（仅开发环境）
- **TRACE**: 最详细调试信息（仅开发环境）

### 环境变量配置
```bash
# 生产环境
export RUST_LOG=info

# 开发环境
export RUST_LOG=debug

# 调试环境
export RUST_LOG=trace
```

## 📁 文件结构

```
media-server/
├── src/
│   ├── main.rs           # 主应用和 API 端点
│   └── logging.rs        # 日志系统核心
├── logs/                 # 日志文件目录
│   └── media-server.log.2025-01-01  # 按天轮转的日志文件
├── test_logging.sh       # 功能测试脚本
└── LOGGING_INTEGRATION.md # 本文档
```

## 🚀 部署和运维

### 1. 启动服务
```bash
# 开发环境
RUST_LOG=debug cargo run

# 生产环境
RUST_LOG=info cargo run --release
```

### 2. 日志轮转
- 自动按天轮转日志文件
- 建议配置日志清理策略（保留30天）
- 可集成 logrotate 等系统工具

### 3. 监控集成
- 可接入 Prometheus 进行指标收集
- 支持 Grafana 仪表板可视化
- 可配置告警规则（错误率、响应时间等）

## 🔧 扩展功能

### 1. 用户行为追踪
```rust
// 记录用户操作
log_with_user!(info, user_id, request_id, "用户 {} 上传了文件 {}", user_id, filename);
```

### 2. 性能监控
```rust
#[instrument]
async fn expensive_operation() {
    let start = Instant::now();
    // 执行操作
    let duration = start.elapsed();
    info!("操作耗时: {:?}", duration);
}
```

### 3. 错误告警
```rust
if ERROR_COUNT.load(Ordering::Relaxed) > 100 {
    warn!("错误数量过高，需要关注");
    // 发送告警通知
}
```

## 📈 最佳实践

1. **结构化日志**: 使用一致的字段名和格式
2. **适度记录**: 避免过度日志记录影响性能
3. **敏感信息**: 不要记录密码、令牌等敏感数据
4. **错误上下文**: 记录足够的上下文信息便于调试
5. **定期清理**: 设置日志保留策略避免磁盘空间不足

## 🎉 总结

通过整合日志、错误监控和查询功能，我们构建了一个完整的可观测性系统：

- ✅ **多层次日志记录**: 控制台 + 文件 + 内存
- ✅ **实时错误监控**: 计数器 + 指标端点
- ✅ **灵活查询系统**: 多维度过滤 + 分页支持
- ✅ **生产就绪**: 日志轮转 + 性能优化
- ✅ **易于扩展**: 模块化设计 + 标准接口

这套系统为应用的运维、调试和监控提供了强有力的支持，确保系统的可靠性和可维护性。