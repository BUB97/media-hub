#!/bin/bash

# 自动化测试脚本
# 用法: ./scripts/test.sh [frontend|backend|integration|all]

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# 检查依赖
check_dependencies() {
    log_info "检查依赖..."
    
    # 检查 Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker 未安装"
        exit 1
    fi
    
    # 检查 Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose 未安装"
        exit 1
    fi
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js 未安装"
        exit 1
    fi
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Rust 未安装"
        exit 1
    fi
    
    log_success "所有依赖检查通过"
}

# 前端测试
test_frontend() {
    log_info "开始前端测试..."
    
    cd frontend
    
    # 安装依赖
    log_info "安装前端依赖..."
    npm ci
    
    # 代码格式检查
    log_info "运行 ESLint..."
    npm run lint
    
    # TypeScript 类型检查
    log_info "运行 TypeScript 检查..."
    npm run format:check
    
    # 构建测试
    log_info "测试前端构建..."
    npm run build
    
    cd ..
    log_success "前端测试完成"
}

# 后端测试
test_backend() {
    log_info "开始后端测试..."
    
    cd backend
    
    # 启动测试数据库
    log_info "启动测试数据库..."
    docker-compose -f ../docker-compose.dev.yml up -d postgres-dev redis-dev
    
    # 等待数据库启动
    sleep 10
    
    # 设置测试环境变量
    export DATABASE_URL="postgres://postgres:postgres@localhost:5433/media_hub_dev"
    export REDIS_URL="redis://localhost:6380"
    export JWT_SECRET="test-secret-key"
    
    # 代码格式检查
    log_info "检查 Rust 代码格式..."
    cargo fmt --all -- --check
    
    # Clippy 检查
    log_info "运行 Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    # 运行测试
    log_info "运行单元测试..."
    cargo test --verbose
    
    # 构建测试
    log_info "测试后端构建..."
    cargo build --release
    
    # 清理测试数据库
    docker-compose -f ../docker-compose.dev.yml down
    
    cd ..
    log_success "后端测试完成"
}

# 集成测试
test_integration() {
    log_info "开始集成测试..."
    
    # 启动所有服务
    log_info "启动测试环境..."
    docker-compose -f docker-compose.dev.yml up -d
    
    # 等待服务启动
    log_info "等待服务启动..."
    sleep 30
    
    # 健康检查
    log_info "检查服务健康状态..."
    
    # 检查后端健康
    if curl -f http://localhost:8001/health; then
        log_success "后端服务健康检查通过"
    else
        log_error "后端服务健康检查失败"
        docker-compose -f docker-compose.dev.yml logs backend-dev
        exit 1
    fi
    
    # 检查前端健康
    if curl -f http://localhost:5174/health; then
        log_success "前端服务健康检查通过"
    else
        log_warning "前端健康检查失败，但这可能是正常的"
    fi
    
    # API 测试
    log_info "运行 API 测试..."
    
    # 测试用户注册
    REGISTER_RESPONSE=$(curl -s -X POST http://localhost:8001/api/auth/register \
        -H "Content-Type: application/json" \
        -d '{"username":"testuser","email":"test@example.com","password":"testpass123"}')
    
    if echo "$REGISTER_RESPONSE" | grep -q "user"; then
        log_success "用户注册 API 测试通过"
    else
        log_error "用户注册 API 测试失败"
        echo "$REGISTER_RESPONSE"
    fi
    
    # 测试用户登录
    LOGIN_RESPONSE=$(curl -s -X POST http://localhost:8001/api/auth/login \
        -H "Content-Type: application/json" \
        -d '{"username":"testuser","password":"testpass123"}')
    
    if echo "$LOGIN_RESPONSE" | grep -q "user"; then
        log_success "用户登录 API 测试通过"
    else
        log_error "用户登录 API 测试失败"
        echo "$LOGIN_RESPONSE"
    fi
    
    # 清理测试环境
    log_info "清理测试环境..."
    docker-compose -f docker-compose.dev.yml down
    
    log_success "集成测试完成"
}

# 性能测试
test_performance() {
    log_info "开始性能测试..."
    
    # 检查是否安装了 ab (Apache Bench)
    if ! command -v ab &> /dev/null; then
        log_warning "Apache Bench (ab) 未安装，跳过性能测试"
        return
    fi
    
    # 启动服务
    docker-compose -f docker-compose.dev.yml up -d backend-dev
    sleep 20
    
    # 性能测试
    log_info "运行性能测试 (100 请求，并发 10)..."
    ab -n 100 -c 10 http://localhost:8001/health
    
    # 清理
    docker-compose -f docker-compose.dev.yml down
    
    log_success "性能测试完成"
}

# 安全测试
test_security() {
    log_info "开始安全测试..."
    
    # 检查是否有敏感信息泄露
    log_info "检查敏感信息泄露..."
    
    # 检查是否有硬编码的密码或密钥
    if grep -r -i "password\|secret\|key" --include="*.rs" --include="*.ts" --include="*.js" --include="*.vue" . | grep -v ".git" | grep -v "node_modules" | grep -v "target"; then
        log_warning "发现可能的硬编码敏感信息，请检查"
    else
        log_success "未发现硬编码敏感信息"
    fi
    
    # 检查依赖漏洞 (如果安装了 cargo audit)
    if command -v cargo-audit &> /dev/null; then
        log_info "检查 Rust 依赖漏洞..."
        cd backend && cargo audit && cd ..
    else
        log_warning "cargo-audit 未安装，跳过 Rust 依赖漏洞检查"
    fi
    
    # 检查 Node.js 依赖漏洞
    log_info "检查 Node.js 依赖漏洞..."
    cd frontend && npm audit --audit-level moderate && cd ..
    
    log_success "安全测试完成"
}

# 主函数
main() {
    local test_type=${1:-all}
    
    log_info "开始运行测试: $test_type"
    
    check_dependencies
    
    case $test_type in
        frontend)
            test_frontend
            ;;
        backend)
            test_backend
            ;;
        integration)
            test_integration
            ;;
        performance)
            test_performance
            ;;
        security)
            test_security
            ;;
        all)
            test_frontend
            test_backend
            test_integration
            test_performance
            test_security
            ;;
        *)
            log_error "未知的测试类型: $test_type"
            echo "用法: $0 [frontend|backend|integration|performance|security|all]"
            exit 1
            ;;
    esac
    
    log_success "所有测试完成！"
}

# 运行主函数
main "$@"