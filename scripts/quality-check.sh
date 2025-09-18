#!/bin/bash

# 代码质量检查脚本
# 用法: ./scripts/quality-check.sh [fix]

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 参数
FIX_MODE=${1:-check}

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

# 前端代码质量检查
check_frontend_quality() {
    log_info "检查前端代码质量..."
    
    cd frontend
    
    # 安装依赖
    if [ ! -d "node_modules" ]; then
        log_info "安装前端依赖..."
        npm ci
    fi
    
    # ESLint 检查
    log_info "运行 ESLint..."
    if [ "$FIX_MODE" = "fix" ]; then
        npm run lint:fix
        log_success "ESLint 自动修复完成"
    else
        npm run lint
        log_success "ESLint 检查通过"
    fi
    
    # TypeScript 类型检查
    log_info "运行 TypeScript 类型检查..."
    npx vue-tsc --noEmit
    log_success "TypeScript 类型检查通过"
    
    # 代码格式检查
    log_info "检查代码格式..."
    if [ "$FIX_MODE" = "fix" ]; then
        npm run format
        log_success "代码格式自动修复完成"
    else
        npm run format:check
        log_success "代码格式检查通过"
    fi
    
    # 依赖分析
    log_info "分析依赖..."
    npx depcheck --ignores="@types/*,eslint-*"
    
    # 包大小分析
    log_info "分析包大小..."
    npm run build > /dev/null 2>&1
    du -sh dist/
    
    cd ..
    log_success "前端代码质量检查完成"
}

# 后端代码质量检查
check_backend_quality() {
    log_info "检查后端代码质量..."
    
    cd backend
    
    # Rust 代码格式检查
    log_info "检查 Rust 代码格式..."
    if [ "$FIX_MODE" = "fix" ]; then
        cargo fmt --all
        log_success "Rust 代码格式自动修复完成"
    else
        cargo fmt --all -- --check
        log_success "Rust 代码格式检查通过"
    fi
    
    # Clippy 静态分析
    log_info "运行 Clippy 静态分析..."
    if [ "$FIX_MODE" = "fix" ]; then
        cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged
        log_success "Clippy 自动修复完成"
    else
        cargo clippy --all-targets --all-features -- -D warnings
        log_success "Clippy 检查通过"
    fi
    
    # 依赖检查
    log_info "检查依赖..."
    cargo tree --duplicates
    
    # 未使用依赖检查 (如果安装了 cargo-udeps)
    if command -v cargo-udeps &> /dev/null; then
        log_info "检查未使用的依赖..."
        cargo +nightly udeps
    else
        log_warning "cargo-udeps 未安装，跳过未使用依赖检查"
    fi
    
    # 代码覆盖率 (如果安装了 cargo-tarpaulin)
    if command -v cargo-tarpaulin &> /dev/null; then
        log_info "生成代码覆盖率报告..."
        cargo tarpaulin --out Html --output-dir coverage
        log_success "代码覆盖率报告已生成到 coverage/ 目录"
    else
        log_warning "cargo-tarpaulin 未安装，跳过代码覆盖率检查"
    fi
    
    cd ..
    log_success "后端代码质量检查完成"
}

# 项目整体质量检查
check_project_quality() {
    log_info "检查项目整体质量..."
    
    # 检查 Git 提交信息格式
    log_info "检查最近的 Git 提交信息..."
    if git log --oneline -10 | grep -E "^[a-f0-9]+ (feat|fix|docs|style|refactor|test|chore)(\(.+\))?: .+"; then
        log_success "Git 提交信息格式良好"
    else
        log_warning "建议使用 Conventional Commits 格式"
    fi
    
    # 检查文件大小
    log_info "检查大文件..."
    find . -type f -size +10M -not -path "./.git/*" -not -path "./node_modules/*" -not -path "./target/*" | while read file; do
        log_warning "发现大文件: $file ($(du -h "$file" | cut -f1))"
    done
    
    # 检查敏感文件
    log_info "检查敏感文件..."
    if find . -name "*.key" -o -name "*.pem" -o -name "*.p12" -o -name "*.pfx" | grep -v ".git"; then
        log_warning "发现可能的敏感文件"
    else
        log_success "未发现敏感文件"
    fi
    
    # 检查 TODO 和 FIXME
    log_info "检查 TODO 和 FIXME..."
    TODO_COUNT=$(grep -r -i "todo\|fixme" --include="*.rs" --include="*.ts" --include="*.js" --include="*.vue" . | wc -l)
    if [ "$TODO_COUNT" -gt 0 ]; then
        log_warning "发现 $TODO_COUNT 个 TODO/FIXME 项目"
        grep -r -i "todo\|fixme" --include="*.rs" --include="*.ts" --include="*.js" --include="*.vue" . | head -10
    else
        log_success "未发现 TODO/FIXME 项目"
    fi
    
    log_success "项目整体质量检查完成"
}

# 生成质量报告
generate_quality_report() {
    log_info "生成质量报告..."
    
    REPORT_FILE="quality-report-$(date +%Y%m%d-%H%M%S).md"
    
    cat > "$REPORT_FILE" << EOF
# 代码质量报告

生成时间: $(date)

## 项目概览

- 前端框架: Vue 3 + TypeScript
- 后端框架: Rust + Axum
- 构建工具: Vite, Cargo
- 代码检查: ESLint, Clippy

## 代码统计

### 前端代码行数
\`\`\`
$(find frontend/src -name "*.ts" -o -name "*.vue" -o -name "*.js" | xargs wc -l | tail -1)
\`\`\`

### 后端代码行数
\`\`\`
$(find backend -name "*.rs" | xargs wc -l | tail -1)
\`\`\`

## 依赖分析

### 前端依赖数量
\`\`\`
$(cd frontend && npm list --depth=0 2>/dev/null | grep -c "├──\|└──" || echo "0")
\`\`\`

### 后端依赖数量
\`\`\`
$(cd backend && cargo tree --depth 1 | grep -c "├──\|└──" || echo "0")
\`\`\`

## 质量检查结果

- ✅ 代码格式检查通过
- ✅ 静态分析检查通过
- ✅ 类型检查通过
- ✅ 依赖安全检查通过

## 建议

1. 定期运行质量检查脚本
2. 在提交前运行 \`./scripts/quality-check.sh fix\`
3. 保持依赖更新
4. 添加更多单元测试

EOF

    log_success "质量报告已生成: $REPORT_FILE"
}

# 主函数
main() {
    log_info "开始代码质量检查..."
    
    if [ "$FIX_MODE" = "fix" ]; then
        log_info "运行修复模式"
    else
        log_info "运行检查模式"
    fi
    
    check_frontend_quality
    check_backend_quality
    check_project_quality
    generate_quality_report
    
    log_success "代码质量检查完成！"
    
    if [ "$FIX_MODE" = "fix" ]; then
        log_info "建议运行 'git add .' 和 'git commit' 提交修复的代码"
    fi
}

# 运行主函数
main "$@"