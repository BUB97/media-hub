#!/bin/bash

# 开发环境设置脚本
# 用法: ./scripts/setup-dev.sh

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

# 检查系统要求
check_system_requirements() {
    log_info "检查系统要求..."
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js 未安装，请先安装 Node.js 18+"
        exit 1
    fi
    
    NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$NODE_VERSION" -lt 18 ]; then
        log_error "Node.js 版本过低，需要 18+，当前版本: $(node --version)"
        exit 1
    fi
    log_success "Node.js 版本检查通过: $(node --version)"
    
    # 检查 Rust
    if ! command -v rustc &> /dev/null; then
        log_error "Rust 未安装，请先安装 Rust"
        log_info "安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    log_success "Rust 版本检查通过: $(rustc --version)"
    
    # 检查 Docker
    if ! command -v docker &> /dev/null; then
        log_warning "Docker 未安装，部分功能可能不可用"
    else
        log_success "Docker 版本检查通过: $(docker --version)"
    fi
    
    # 检查 Git
    if ! command -v git &> /dev/null; then
        log_error "Git 未安装，请先安装 Git"
        exit 1
    fi
    log_success "Git 版本检查通过: $(git --version)"
}

# 安装 Rust 工具
install_rust_tools() {
    log_info "安装 Rust 开发工具..."
    
    # 安装 cargo-watch (热重载)
    if ! command -v cargo-watch &> /dev/null; then
        log_info "安装 cargo-watch..."
        cargo install cargo-watch
        log_success "cargo-watch 安装完成"
    else
        log_success "cargo-watch 已安装"
    fi
    
    # 安装 cargo-tarpaulin (代码覆盖率)
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_info "安装 cargo-tarpaulin..."
        cargo install cargo-tarpaulin
        log_success "cargo-tarpaulin 安装完成"
    else
        log_success "cargo-tarpaulin 已安装"
    fi
    
    # 安装 cargo-udeps (未使用依赖检查)
    if ! command -v cargo-udeps &> /dev/null; then
        log_info "安装 cargo-udeps..."
        cargo install cargo-udeps --locked
        log_success "cargo-udeps 安装完成"
    else
        log_success "cargo-udeps 已安装"
    fi
    
    # 安装 cargo-audit (安全审计)
    if ! command -v cargo-audit &> /dev/null; then
        log_info "安装 cargo-audit..."
        cargo install cargo-audit
        log_success "cargo-audit 安装完成"
    else
        log_success "cargo-audit 已安装"
    fi
    
    # 添加 nightly 工具链 (用于 cargo-udeps)
    log_info "添加 Rust nightly 工具链..."
    rustup toolchain install nightly
    log_success "Rust nightly 工具链安装完成"
}

# 设置前端环境
setup_frontend() {
    log_info "设置前端开发环境..."
    
    cd frontend
    
    # 安装依赖
    log_info "安装前端依赖..."
    npm ci
    log_success "前端依赖安装完成"
    
    # 检查 package.json 脚本
    if ! npm run --silent 2>/dev/null | grep -q "lint"; then
        log_warning "package.json 中缺少 lint 脚本"
    fi
    
    cd ..
}

# 设置后端环境
setup_backend() {
    log_info "设置后端开发环境..."
    
    cd backend
    
    # 检查依赖
    log_info "检查后端依赖..."
    cargo check
    log_success "后端依赖检查完成"
    
    # 预构建依赖
    log_info "预构建后端依赖..."
    cargo build
    log_success "后端依赖构建完成"
    
    cd ..
}

# 设置环境变量
setup_environment() {
    log_info "设置环境变量..."
    
    if [ ! -f ".env" ]; then
        if [ -f ".env.example" ]; then
            log_info "复制环境变量示例文件..."
            cp .env.example .env
            log_success "已创建 .env 文件，请根据需要修改配置"
        else
            log_warning ".env.example 文件不存在"
        fi
    else
        log_success ".env 文件已存在"
    fi
}

# 设置 Git hooks
setup_git_hooks() {
    log_info "设置 Git hooks..."
    
    # 创建 pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

echo "运行 pre-commit 检查..."

# 运行质量检查
if [ -f "scripts/quality-check.sh" ]; then
    ./scripts/quality-check.sh
    if [ $? -ne 0 ]; then
        echo "质量检查失败，提交被阻止"
        exit 1
    fi
fi

echo "pre-commit 检查通过"
EOF
    
    chmod +x .git/hooks/pre-commit
    log_success "Git pre-commit hook 设置完成"
    
    # 创建 commit-msg hook (检查提交信息格式)
    cat > .git/hooks/commit-msg << 'EOF'
#!/bin/bash

commit_regex='^(feat|fix|docs|style|refactor|test|chore)(\(.+\))?: .{1,50}'

if ! grep -qE "$commit_regex" "$1"; then
    echo "提交信息格式不正确！"
    echo "请使用 Conventional Commits 格式："
    echo "  feat: 新功能"
    echo "  fix: 修复bug"
    echo "  docs: 文档更新"
    echo "  style: 代码格式"
    echo "  refactor: 重构"
    echo "  test: 测试"
    echo "  chore: 构建/工具"
    echo ""
    echo "示例: feat(auth): 添加用户登录功能"
    exit 1
fi
EOF
    
    chmod +x .git/hooks/commit-msg
    log_success "Git commit-msg hook 设置完成"
}

# 创建开发脚本
create_dev_scripts() {
    log_info "创建开发脚本..."
    
    # 创建启动脚本
    cat > scripts/start-dev.sh << 'EOF'
#!/bin/bash

# 启动开发环境
echo "启动开发环境..."

# 启动数据库和缓存
docker-compose -f docker-compose.dev.yml up -d postgres redis

# 等待服务启动
sleep 5

# 启动后端 (在后台)
cd backend && cargo watch -x run &
BACKEND_PID=$!

# 启动前端 (在后台)
cd ../frontend && npm run dev &
FRONTEND_PID=$!

echo "开发环境已启动"
echo "前端: http://localhost:5173"
echo "后端: http://localhost:8000"
echo "按 Ctrl+C 停止所有服务"

# 等待中断信号
trap "kill $BACKEND_PID $FRONTEND_PID; docker-compose -f docker-compose.dev.yml down; exit" INT
wait
EOF
    
    chmod +x scripts/start-dev.sh
    log_success "开发启动脚本创建完成"
    
    # 创建停止脚本
    cat > scripts/stop-dev.sh << 'EOF'
#!/bin/bash

echo "停止开发环境..."

# 停止 Docker 服务
docker-compose -f docker-compose.dev.yml down

# 停止可能运行的进程
pkill -f "cargo watch" || true
pkill -f "npm run dev" || true

echo "开发环境已停止"
EOF
    
    chmod +x scripts/stop-dev.sh
    log_success "开发停止脚本创建完成"
}

# 设置 IDE 配置
setup_ide_config() {
    log_info "设置 IDE 配置..."
    
    # VS Code 配置
    mkdir -p .vscode
    
    cat > .vscode/settings.json << 'EOF'
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "files.associations": {
    "*.rs": "rust"
  },
  "typescript.preferences.importModuleSpecifier": "relative"
}
EOF
    
    cat > .vscode/extensions.json << 'EOF'
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "Vue.volar",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "dbaeumer.vscode-eslint",
    "ms-vscode.vscode-typescript-next"
  ]
}
EOF
    
    log_success "VS Code 配置完成"
}

# 运行初始测试
run_initial_tests() {
    log_info "运行初始测试..."
    
    # 前端测试
    cd frontend
    if npm run lint --silent 2>/dev/null; then
        log_success "前端 lint 检查通过"
    else
        log_warning "前端 lint 检查失败，请检查代码"
    fi
    cd ..
    
    # 后端测试
    cd backend
    if cargo clippy --all-targets --all-features -- -D warnings; then
        log_success "后端 clippy 检查通过"
    else
        log_warning "后端 clippy 检查失败，请检查代码"
    fi
    cd ..
}

# 显示开发指南
show_dev_guide() {
    log_success "开发环境设置完成！"
    echo ""
    echo "🚀 快速开始："
    echo "  ./scripts/start-dev.sh    # 启动开发环境"
    echo "  ./scripts/stop-dev.sh     # 停止开发环境"
    echo "  ./scripts/test.sh         # 运行测试"
    echo "  ./scripts/quality-check.sh # 代码质量检查"
    echo ""
    echo "📁 项目结构："
    echo "  frontend/     # Vue 3 前端应用"
    echo "  backend/      # Rust 后端服务"
    echo "  scripts/      # 开发脚本"
    echo ""
    echo "🔧 开发工具："
    echo "  前端: http://localhost:5173"
    echo "  后端: http://localhost:8000"
    echo "  数据库: localhost:5432"
    echo "  Redis: localhost:6379"
    echo ""
    echo "📖 更多信息请查看 README.md"
}

# 主函数
main() {
    log_info "开始设置开发环境..."
    
    check_system_requirements
    install_rust_tools
    setup_frontend
    setup_backend
    setup_environment
    setup_git_hooks
    create_dev_scripts
    setup_ide_config
    run_initial_tests
    show_dev_guide
}

# 运行主函数
main "$@"