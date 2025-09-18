# CI/CD 集成指南

本项目已集成完整的 CI/CD 功能，包括自动化构建、测试、部署和监控。

## 🚀 快速开始

### 1. 开发环境设置

```bash
# 设置开发环境
./scripts/setup-dev.sh

# 启动开发环境
./scripts/start-dev.sh

# 停止开发环境
./scripts/stop-dev.sh
```

### 2. 代码质量检查

```bash
# 运行质量检查
./scripts/quality-check.sh

# 自动修复代码问题
./scripts/quality-check.sh fix
```

### 3. 运行测试

```bash
# 运行所有测试
./scripts/test.sh

# 运行特定类型的测试
./scripts/test.sh frontend    # 前端测试
./scripts/test.sh backend     # 后端测试
./scripts/test.sh integration # 集成测试
```

## 📦 部署流程

### 预发布环境部署

```bash
# 构建并部署到预发布环境
./scripts/deploy.sh staging deploy

# 查看部署状态
./scripts/deploy.sh staging status

# 运行性能测试
./scripts/deploy.sh staging test
```

### 生产环境部署

```bash
# 构建生产环境镜像
./scripts/deploy.sh production build

# 部署到生产环境
./scripts/deploy.sh production deploy

# 回滚部署 (如果需要)
./scripts/deploy.sh production rollback
```

## 🔄 GitHub Actions 工作流

项目包含以下自动化工作流：

### 1. 持续集成 (CI)
- **触发条件**: Push 到 main/develop 分支，Pull Request
- **执行内容**:
  - 前端构建和测试
  - 后端构建和测试
  - 代码质量检查
  - 安全扫描
  - 集成测试

### 2. 持续部署 (CD)
- **触发条件**: Push 到 main 分支 (生产环境)，Push 到 develop 分支 (预发布环境)
- **执行内容**:
  - 构建 Docker 镜像
  - 推送到镜像仓库
  - 部署到目标环境
  - 健康检查
  - 通知部署结果

### 3. 定时任务
- **数据库备份**: 每日凌晨 2 点
- **安全扫描**: 每周一次
- **依赖更新检查**: 每周一次

## 🐳 Docker 配置

### 开发环境

```bash
# 启动开发环境 (包含热重载)
docker-compose -f docker-compose.dev.yml up -d

# 查看日志
docker-compose -f docker-compose.dev.yml logs -f
```

### 生产环境

```bash
# 启动生产环境
docker-compose up -d

# 扩展服务
docker-compose up -d --scale backend=3
```

## 🔧 环境配置

### 环境变量文件

- `.env.example` - 环境变量模板
- `.env.staging` - 预发布环境配置
- `.env.production` - 生产环境配置

### 重要配置项

```bash
# 数据库配置
DATABASE_URL=postgresql://user:password@host:port/database

# Redis 配置
REDIS_URL=redis://password@host:port/db

# JWT 配置
JWT_SECRET=your-secret-key

# 腾讯云 COS 配置
COS_SECRET_ID=your-secret-id
COS_SECRET_KEY=your-secret-key
COS_BUCKET=your-bucket-name
```

## 📊 监控和日志

### 健康检查端点

- 前端: `http://localhost:3000/health`
- 后端: `http://localhost:8000/health`
- 数据库: `http://localhost:8000/health/db`
- Redis: `http://localhost:8000/health/redis`

### 日志查看

```bash
# 查看应用日志
docker-compose logs -f backend frontend

# 查看特定服务日志
docker-compose logs -f backend

# 查看实时日志
docker-compose logs -f --tail=100 backend
```

### 性能监控

```bash
# 查看资源使用情况
docker stats

# 运行性能测试
./scripts/deploy.sh staging test
```

## 💾 数据备份和恢复

### 备份数据

```bash
# 完整备份
./scripts/backup.sh backup production

# 仅备份数据库
./scripts/backup.sh backup staging

# 列出备份文件
./scripts/backup.sh list production
```

### 恢复数据

```bash
# 恢复数据库
./scripts/backup.sh restore staging backups/db-staging-20240101-120000.sql.gz

# 恢复 Redis
./scripts/backup.sh restore staging backups/redis-staging-20240101-120000.rdb.gz
```

### 清理旧备份

```bash
# 清理 30 天前的备份
./scripts/backup.sh cleanup production
```

## 🔒 安全最佳实践

### 1. 密钥管理
- 使用强密码和随机密钥
- 定期轮换密钥
- 不要在代码中硬编码密钥
- 使用环境变量或密钥管理服务

### 2. 网络安全
- 启用 HTTPS
- 配置防火墙规则
- 使用 VPN 访问生产环境
- 限制数据库访问

### 3. 容器安全
- 使用非 root 用户运行容器
- 定期更新基础镜像
- 扫描镜像漏洞
- 限制容器权限

## 🚨 故障排除

### 常见问题

1. **构建失败**
   ```bash
   # 清理 Docker 缓存
   docker system prune -a
   
   # 重新构建镜像
   docker-compose build --no-cache
   ```

2. **数据库连接失败**
   ```bash
   # 检查数据库状态
   docker-compose ps postgres
   
   # 查看数据库日志
   docker-compose logs postgres
   ```

3. **前端无法访问后端**
   ```bash
   # 检查网络配置
   docker network ls
   
   # 检查服务端口
   docker-compose ps
   ```

### 日志分析

```bash
# 查看错误日志
docker-compose logs --tail=100 | grep ERROR

# 查看特定时间段的日志
docker-compose logs --since="2024-01-01T00:00:00" --until="2024-01-01T23:59:59"
```

## 📈 性能优化

### 1. 数据库优化
- 添加适当的索引
- 优化查询语句
- 配置连接池
- 定期分析表

### 2. 缓存策略
- 使用 Redis 缓存热点数据
- 配置 HTTP 缓存头
- 使用 CDN 加速静态资源

### 3. 应用优化
- 启用 gzip 压缩
- 优化镜像大小
- 使用多阶段构建
- 配置资源限制

## 🔄 版本管理

### Git 工作流

1. **功能开发**: 从 `develop` 分支创建功能分支
2. **代码审查**: 创建 Pull Request 到 `develop`
3. **预发布**: 合并到 `develop` 分支自动部署到预发布环境
4. **生产发布**: 合并到 `main` 分支自动部署到生产环境

### 版本标签

```bash
# 创建版本标签
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# 查看版本历史
git tag -l
```

## 📞 支持和联系

如果在使用 CI/CD 功能时遇到问题，请：

1. 查看本文档的故障排除部分
2. 检查 GitHub Actions 的执行日志
3. 查看应用程序日志
4. 联系开发团队获取支持

---

**注意**: 在生产环境中使用前，请确保：
- 所有密钥和密码都已更改为强密码
- 网络安全配置已正确设置
- 备份策略已经过测试
- 监控和告警已配置完成