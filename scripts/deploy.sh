#!/bin/bash

# 生产环境部署脚本
# 用法: ./scripts/deploy.sh [environment] [action]
# 环境: staging, production
# 操作: build, deploy, rollback, status

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 参数
ENVIRONMENT=${1:-staging}
ACTION=${2:-deploy}

# 配置
PROJECT_NAME="media-hub"
DOCKER_REGISTRY="your-registry.com"
BACKUP_DIR="./backups"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查环境
check_environment() {
    log_info "检查部署环境: $ENVIRONMENT"
    
    if [ "$ENVIRONMENT" != "staging" ] && [ "$ENVIRONMENT" != "production" ]; then
        log_error "无效的环境: $ENVIRONMENT (支持: staging, production)"
        exit 1
    fi
    
    # 检查必要工具
    for tool in docker docker-compose git; do
        if ! command -v $tool &> /dev/null; then
            log_error "$tool 未安装"
            exit 1
        fi
    done
    
    # 检查环境变量文件
    ENV_FILE=".env.$ENVIRONMENT"
    if [ ! -f "$ENV_FILE" ]; then
        log_error "环境变量文件不存在: $ENV_FILE"
        exit 1
    fi
    
    log_success "环境检查通过"
}

# 构建镜像
build_images() {
    log_info "构建 Docker 镜像..."
    
    # 获取版本号
    VERSION=$(git describe --tags --always --dirty)
    COMMIT_HASH=$(git rev-parse --short HEAD)
    BUILD_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    log_info "版本信息: $VERSION ($COMMIT_HASH)"
    
    # 构建前端镜像
    log_info "构建前端镜像..."
    docker build \
        --build-arg VERSION="$VERSION" \
        --build-arg COMMIT_HASH="$COMMIT_HASH" \
        --build-arg BUILD_TIME="$BUILD_TIME" \
        -t "$DOCKER_REGISTRY/$PROJECT_NAME-frontend:$VERSION" \
        -t "$DOCKER_REGISTRY/$PROJECT_NAME-frontend:latest" \
        -f frontend/Dockerfile \
        .
    
    # 构建后端镜像
    log_info "构建后端镜像..."
    docker build \
        --build-arg VERSION="$VERSION" \
        --build-arg COMMIT_HASH="$COMMIT_HASH" \
        --build-arg BUILD_TIME="$BUILD_TIME" \
        -t "$DOCKER_REGISTRY/$PROJECT_NAME-backend:$VERSION" \
        -t "$DOCKER_REGISTRY/$PROJECT_NAME-backend:latest" \
        -f backend/Dockerfile \
        .
    
    log_success "镜像构建完成"
}

# 推送镜像
push_images() {
    log_info "推送镜像到仓库..."
    
    VERSION=$(git describe --tags --always --dirty)
    
    # 推送前端镜像
    docker push "$DOCKER_REGISTRY/$PROJECT_NAME-frontend:$VERSION"
    docker push "$DOCKER_REGISTRY/$PROJECT_NAME-frontend:latest"
    
    # 推送后端镜像
    docker push "$DOCKER_REGISTRY/$PROJECT_NAME-backend:$VERSION"
    docker push "$DOCKER_REGISTRY/$PROJECT_NAME-backend:latest"
    
    log_success "镜像推送完成"
}

# 数据库备份
backup_database() {
    log_info "备份数据库..."
    
    # 创建备份目录
    mkdir -p "$BACKUP_DIR"
    
    # 备份文件名
    BACKUP_FILE="$BACKUP_DIR/db-backup-$ENVIRONMENT-$(date +%Y%m%d-%H%M%S).sql"
    
    # 从环境变量文件读取数据库配置
    source ".env.$ENVIRONMENT"
    
    # 执行备份
    docker exec -i $(docker-compose -f docker-compose.yml ps -q postgres) \
        pg_dump -U "$POSTGRES_USER" "$POSTGRES_DB" > "$BACKUP_FILE"
    
    # 压缩备份文件
    gzip "$BACKUP_FILE"
    
    log_success "数据库备份完成: $BACKUP_FILE.gz"
    
    # 清理旧备份 (保留最近10个)
    ls -t "$BACKUP_DIR"/db-backup-$ENVIRONMENT-*.sql.gz | tail -n +11 | xargs -r rm
}

# 部署应用
deploy_application() {
    log_info "部署应用到 $ENVIRONMENT 环境..."
    
    # 备份数据库 (生产环境)
    if [ "$ENVIRONMENT" = "production" ]; then
        backup_database
    fi
    
    # 拉取最新镜像
    VERSION=$(git describe --tags --always --dirty)
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" pull
    
    # 停止旧服务 (滚动更新)
    log_info "执行滚动更新..."
    
    # 更新后端服务
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" \
        up -d --no-deps backend
    
    # 等待后端服务启动
    sleep 10
    
    # 健康检查
    if ! health_check; then
        log_error "后端服务健康检查失败，回滚部署"
        rollback_deployment
        exit 1
    fi
    
    # 更新前端服务
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" \
        up -d --no-deps frontend nginx
    
    # 清理未使用的镜像
    docker image prune -f
    
    log_success "应用部署完成"
}

# 健康检查
health_check() {
    log_info "执行健康检查..."
    
    # 从环境变量文件读取配置
    source ".env.$ENVIRONMENT"
    
    # 检查后端服务
    for i in {1..30}; do
        if curl -f -s "http://localhost:$BACKEND_PORT/health" > /dev/null; then
            log_success "后端服务健康检查通过"
            return 0
        fi
        log_info "等待后端服务启动... ($i/30)"
        sleep 2
    done
    
    log_error "后端服务健康检查失败"
    return 1
}

# 回滚部署
rollback_deployment() {
    log_warning "开始回滚部署..."
    
    # 获取上一个版本
    PREVIOUS_VERSION=$(git describe --tags --abbrev=0 HEAD~1 2>/dev/null || echo "latest")
    
    log_info "回滚到版本: $PREVIOUS_VERSION"
    
    # 更新镜像标签到上一个版本
    export FRONTEND_IMAGE="$DOCKER_REGISTRY/$PROJECT_NAME-frontend:$PREVIOUS_VERSION"
    export BACKEND_IMAGE="$DOCKER_REGISTRY/$PROJECT_NAME-backend:$PREVIOUS_VERSION"
    
    # 重新部署
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" \
        up -d --force-recreate
    
    log_success "回滚完成"
}

# 查看部署状态
check_status() {
    log_info "检查部署状态..."
    
    # 显示服务状态
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" ps
    
    # 显示资源使用情况
    echo ""
    log_info "资源使用情况:"
    docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}"
    
    # 显示最近的日志
    echo ""
    log_info "最近的日志:"
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" \
        logs --tail=20 --timestamps
}

# 数据库迁移
run_migrations() {
    log_info "运行数据库迁移..."
    
    # 这里应该根据实际的迁移工具进行调整
    # 例如使用 sqlx migrate 或其他迁移工具
    
    docker-compose -f docker-compose.yml --env-file ".env.$ENVIRONMENT" \
        exec backend /app/run-migrations.sh
    
    log_success "数据库迁移完成"
}

# 性能测试
run_performance_test() {
    log_info "运行性能测试..."
    
    source ".env.$ENVIRONMENT"
    
    # 使用 Apache Bench 进行简单的性能测试
    ab -n 1000 -c 10 "http://localhost:$BACKEND_PORT/health"
    
    log_success "性能测试完成"
}

# 监控和告警
setup_monitoring() {
    log_info "设置监控和告警..."
    
    # 这里可以集成 Prometheus, Grafana, AlertManager 等
    # 或者发送部署通知到 Slack, 钉钉等
    
    log_success "监控设置完成"
}

# 显示帮助信息
show_help() {
    echo "用法: $0 [environment] [action]"
    echo ""
    echo "环境:"
    echo "  staging     预发布环境"
    echo "  production  生产环境"
    echo ""
    echo "操作:"
    echo "  build       构建镜像"
    echo "  deploy      部署应用"
    echo "  rollback    回滚部署"
    echo "  status      查看状态"
    echo "  migrate     数据库迁移"
    echo "  test        性能测试"
    echo "  monitor     设置监控"
    echo ""
    echo "示例:"
    echo "  $0 staging deploy    # 部署到预发布环境"
    echo "  $0 production build  # 构建生产环境镜像"
    echo "  $0 production status # 查看生产环境状态"
}

# 主函数
main() {
    case "$ACTION" in
        "build")
            check_environment
            build_images
            push_images
            ;;
        "deploy")
            check_environment
            build_images
            push_images
            deploy_application
            run_migrations
            health_check
            setup_monitoring
            ;;
        "rollback")
            check_environment
            rollback_deployment
            ;;
        "status")
            check_environment
            check_status
            ;;
        "migrate")
            check_environment
            run_migrations
            ;;
        "test")
            check_environment
            run_performance_test
            ;;
        "monitor")
            check_environment
            setup_monitoring
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            log_error "未知操作: $ACTION"
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@"