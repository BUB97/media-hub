#!/bin/bash

# 数据备份和恢复脚本
# 用法: ./scripts/backup.sh [action] [environment] [backup_file]
# 操作: backup, restore, list, cleanup
# 环境: staging, production

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 参数
ACTION=${1:-backup}
ENVIRONMENT=${2:-staging}
BACKUP_FILE=${3:-}

# 配置
BACKUP_DIR="./backups"
S3_BUCKET="media-hub-backups"
RETENTION_DAYS=30

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
    log_info "检查备份环境: $ENVIRONMENT"
    
    if [ "$ENVIRONMENT" != "staging" ] && [ "$ENVIRONMENT" != "production" ]; then
        log_error "无效的环境: $ENVIRONMENT (支持: staging, production)"
        exit 1
    fi
    
    # 检查环境变量文件
    ENV_FILE=".env.$ENVIRONMENT"
    if [ ! -f "$ENV_FILE" ]; then
        log_error "环境变量文件不存在: $ENV_FILE"
        exit 1
    fi
    
    # 创建备份目录
    mkdir -p "$BACKUP_DIR"
    
    log_success "环境检查通过"
}

# 数据库备份
backup_database() {
    log_info "开始数据库备份..."
    
    # 加载环境变量
    source ".env.$ENVIRONMENT"
    
    # 生成备份文件名
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    DB_BACKUP_FILE="$BACKUP_DIR/db-$ENVIRONMENT-$TIMESTAMP.sql"
    
    # 检查数据库容器是否运行
    if ! docker-compose -f docker-compose.yml ps postgres | grep -q "Up"; then
        log_error "PostgreSQL 容器未运行"
        exit 1
    fi
    
    # 执行数据库备份
    log_info "备份数据库到: $DB_BACKUP_FILE"
    docker-compose -f docker-compose.yml exec -T postgres \
        pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" \
        --verbose --clean --if-exists --create > "$DB_BACKUP_FILE"
    
    # 压缩备份文件
    gzip "$DB_BACKUP_FILE"
    DB_BACKUP_FILE="$DB_BACKUP_FILE.gz"
    
    log_success "数据库备份完成: $DB_BACKUP_FILE"
    echo "$DB_BACKUP_FILE"
}

# 文件备份
backup_files() {
    log_info "开始文件备份..."
    
    # 生成备份文件名
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    FILES_BACKUP_FILE="$BACKUP_DIR/files-$ENVIRONMENT-$TIMESTAMP.tar.gz"
    
    # 备份上传的文件 (如果存在)
    if [ -d "uploads" ]; then
        log_info "备份上传文件到: $FILES_BACKUP_FILE"
        tar -czf "$FILES_BACKUP_FILE" uploads/
        log_success "文件备份完成: $FILES_BACKUP_FILE"
        echo "$FILES_BACKUP_FILE"
    else
        log_warning "uploads 目录不存在，跳过文件备份"
    fi
}

# Redis 备份
backup_redis() {
    log_info "开始 Redis 备份..."
    
    # 生成备份文件名
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    REDIS_BACKUP_FILE="$BACKUP_DIR/redis-$ENVIRONMENT-$TIMESTAMP.rdb"
    
    # 检查 Redis 容器是否运行
    if ! docker-compose -f docker-compose.yml ps redis | grep -q "Up"; then
        log_error "Redis 容器未运行"
        exit 1
    fi
    
    # 执行 Redis 备份
    log_info "备份 Redis 到: $REDIS_BACKUP_FILE"
    docker-compose -f docker-compose.yml exec -T redis \
        redis-cli --rdb /tmp/dump.rdb
    
    docker-compose -f docker-compose.yml exec -T redis \
        cat /tmp/dump.rdb > "$REDIS_BACKUP_FILE"
    
    # 压缩备份文件
    gzip "$REDIS_BACKUP_FILE"
    REDIS_BACKUP_FILE="$REDIS_BACKUP_FILE.gz"
    
    log_success "Redis 备份完成: $REDIS_BACKUP_FILE"
    echo "$REDIS_BACKUP_FILE"
}

# 完整备份
full_backup() {
    log_info "开始完整备份..."
    
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    BACKUP_NAME="full-backup-$ENVIRONMENT-$TIMESTAMP"
    BACKUP_MANIFEST="$BACKUP_DIR/$BACKUP_NAME.manifest"
    
    # 创建备份清单
    echo "# 完整备份清单" > "$BACKUP_MANIFEST"
    echo "# 环境: $ENVIRONMENT" >> "$BACKUP_MANIFEST"
    echo "# 时间: $(date)" >> "$BACKUP_MANIFEST"
    echo "# Git 提交: $(git rev-parse HEAD)" >> "$BACKUP_MANIFEST"
    echo "" >> "$BACKUP_MANIFEST"
    
    # 数据库备份
    DB_FILE=$(backup_database)
    echo "database=$DB_FILE" >> "$BACKUP_MANIFEST"
    
    # 文件备份
    FILES_FILE=$(backup_files)
    if [ -n "$FILES_FILE" ]; then
        echo "files=$FILES_FILE" >> "$BACKUP_MANIFEST"
    fi
    
    # Redis 备份
    REDIS_FILE=$(backup_redis)
    echo "redis=$REDIS_FILE" >> "$BACKUP_MANIFEST"
    
    # 配置文件备份
    CONFIG_BACKUP_FILE="$BACKUP_DIR/config-$ENVIRONMENT-$TIMESTAMP.tar.gz"
    tar -czf "$CONFIG_BACKUP_FILE" \
        .env.$ENVIRONMENT \
        docker-compose.yml \
        nginx.conf \
        --ignore-failed-read 2>/dev/null || true
    echo "config=$CONFIG_BACKUP_FILE" >> "$BACKUP_MANIFEST"
    
    log_success "完整备份完成，清单文件: $BACKUP_MANIFEST"
    
    # 上传到 S3 (如果配置了)
    if command -v aws &> /dev/null && [ -n "$S3_BUCKET" ]; then
        upload_to_s3 "$BACKUP_MANIFEST"
    fi
}

# 上传到 S3
upload_to_s3() {
    local manifest_file="$1"
    log_info "上传备份到 S3..."
    
    # 读取清单文件中的所有备份文件
    while IFS='=' read -r key value; do
        if [[ $key != \#* ]] && [ -n "$value" ]; then
            log_info "上传 $value 到 S3..."
            aws s3 cp "$value" "s3://$S3_BUCKET/$(basename "$value")"
        fi
    done < "$manifest_file"
    
    # 上传清单文件
    aws s3 cp "$manifest_file" "s3://$S3_BUCKET/$(basename "$manifest_file")"
    
    log_success "备份已上传到 S3"
}

# 数据库恢复
restore_database() {
    local backup_file="$1"
    
    if [ ! -f "$backup_file" ]; then
        log_error "备份文件不存在: $backup_file"
        exit 1
    fi
    
    log_warning "即将恢复数据库，这将覆盖现有数据！"
    read -p "确认继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "操作已取消"
        exit 0
    fi
    
    # 加载环境变量
    source ".env.$ENVIRONMENT"
    
    log_info "恢复数据库从: $backup_file"
    
    # 如果是压缩文件，先解压
    if [[ "$backup_file" == *.gz ]]; then
        gunzip -c "$backup_file" | docker-compose -f docker-compose.yml exec -T postgres \
            psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"
    else
        docker-compose -f docker-compose.yml exec -T postgres \
            psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" < "$backup_file"
    fi
    
    log_success "数据库恢复完成"
}

# Redis 恢复
restore_redis() {
    local backup_file="$1"
    
    if [ ! -f "$backup_file" ]; then
        log_error "备份文件不存在: $backup_file"
        exit 1
    fi
    
    log_warning "即将恢复 Redis，这将覆盖现有数据！"
    read -p "确认继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "操作已取消"
        exit 0
    fi
    
    log_info "恢复 Redis 从: $backup_file"
    
    # 停止 Redis 服务
    docker-compose -f docker-compose.yml stop redis
    
    # 恢复数据文件
    if [[ "$backup_file" == *.gz ]]; then
        gunzip -c "$backup_file" | docker-compose -f docker-compose.yml exec -T redis \
            sh -c 'cat > /data/dump.rdb'
    else
        docker-compose -f docker-compose.yml exec -T redis \
            sh -c 'cat > /data/dump.rdb' < "$backup_file"
    fi
    
    # 重启 Redis 服务
    docker-compose -f docker-compose.yml start redis
    
    log_success "Redis 恢复完成"
}

# 列出备份文件
list_backups() {
    log_info "备份文件列表 ($ENVIRONMENT):"
    
    if [ ! -d "$BACKUP_DIR" ]; then
        log_warning "备份目录不存在"
        return
    fi
    
    echo ""
    echo "数据库备份:"
    ls -lh "$BACKUP_DIR"/db-$ENVIRONMENT-*.sql.gz 2>/dev/null | \
        awk '{print $9, $5, $6, $7, $8}' || echo "  无数据库备份文件"
    
    echo ""
    echo "文件备份:"
    ls -lh "$BACKUP_DIR"/files-$ENVIRONMENT-*.tar.gz 2>/dev/null | \
        awk '{print $9, $5, $6, $7, $8}' || echo "  无文件备份"
    
    echo ""
    echo "Redis 备份:"
    ls -lh "$BACKUP_DIR"/redis-$ENVIRONMENT-*.rdb.gz 2>/dev/null | \
        awk '{print $9, $5, $6, $7, $8}' || echo "  无 Redis 备份文件"
    
    echo ""
    echo "完整备份清单:"
    ls -lh "$BACKUP_DIR"/full-backup-$ENVIRONMENT-*.manifest 2>/dev/null | \
        awk '{print $9, $5, $6, $7, $8}' || echo "  无完整备份清单"
}

# 清理旧备份
cleanup_backups() {
    log_info "清理 $RETENTION_DAYS 天前的备份文件..."
    
    if [ ! -d "$BACKUP_DIR" ]; then
        log_warning "备份目录不存在"
        return
    fi
    
    # 清理数据库备份
    find "$BACKUP_DIR" -name "db-$ENVIRONMENT-*.sql.gz" -mtime +$RETENTION_DAYS -delete
    
    # 清理文件备份
    find "$BACKUP_DIR" -name "files-$ENVIRONMENT-*.tar.gz" -mtime +$RETENTION_DAYS -delete
    
    # 清理 Redis 备份
    find "$BACKUP_DIR" -name "redis-$ENVIRONMENT-*.rdb.gz" -mtime +$RETENTION_DAYS -delete
    
    # 清理配置备份
    find "$BACKUP_DIR" -name "config-$ENVIRONMENT-*.tar.gz" -mtime +$RETENTION_DAYS -delete
    
    # 清理清单文件
    find "$BACKUP_DIR" -name "full-backup-$ENVIRONMENT-*.manifest" -mtime +$RETENTION_DAYS -delete
    
    log_success "清理完成"
}

# 显示帮助信息
show_help() {
    echo "用法: $0 [action] [environment] [backup_file]"
    echo ""
    echo "操作:"
    echo "  backup      执行完整备份"
    echo "  restore     恢复备份 (需要指定备份文件)"
    echo "  list        列出备份文件"
    echo "  cleanup     清理旧备份文件"
    echo ""
    echo "环境:"
    echo "  staging     预发布环境"
    echo "  production  生产环境"
    echo ""
    echo "示例:"
    echo "  $0 backup production                    # 备份生产环境"
    echo "  $0 restore staging db-staging-xxx.sql.gz  # 恢复预发布数据库"
    echo "  $0 list production                      # 列出生产环境备份"
    echo "  $0 cleanup staging                      # 清理预发布环境旧备份"
}

# 主函数
main() {
    case "$ACTION" in
        "backup")
            check_environment
            full_backup
            ;;
        "restore")
            if [ -z "$BACKUP_FILE" ]; then
                log_error "请指定备份文件"
                show_help
                exit 1
            fi
            check_environment
            if [[ "$BACKUP_FILE" == *"db-"* ]]; then
                restore_database "$BACKUP_FILE"
            elif [[ "$BACKUP_FILE" == *"redis-"* ]]; then
                restore_redis "$BACKUP_FILE"
            else
                log_error "不支持的备份文件类型"
                exit 1
            fi
            ;;
        "list")
            check_environment
            list_backups
            ;;
        "cleanup")
            check_environment
            cleanup_backups
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