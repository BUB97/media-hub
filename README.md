# Media Hub

> 🚧 **开发状态** - 此项目正在积极开发中，功能和 API 可能会发生变化

一个基于 Rust + Vue 3 的现代化媒体管理平台，集成了企业级 JWT 认证系统和 WASM 媒体处理能力。

## ✨ 特性

### 🔐 安全认证
- **企业级 JWT 认证系统**：HttpOnly Cookie + 双层存储策略
- **智能认证状态管理**：同步检查 + 异步验证平衡性能与准确性
- **多重安全防护**：bcrypt 密码哈希 + JWT 签名 + CSRF 保护

### 🏗️ 现代化架构
- **统一域名架构**：Nginx 反向代理解决跨域问题
- **无状态后端**：支持水平扩展的分布式部署
- **类型安全**：全栈 TypeScript + Rust 类型保障

### 🚀 高性能
- **WASM 媒体处理**：浏览器端高性能媒体文件处理
- **智能缓存**：静态资源长期缓存 + API 响应优化
- **异步架构**：Tokio 异步运行时 + Vue 3 Composition API

### ☁️ 云存储集成
- **腾讯云 COS 对象存储**：企业级云存储解决方案
- **STS 临时凭证**：安全的临时访问控制，避免密钥泄露风险
- **直传架构**：客户端直接上传到 COS，减轻服务器压力
- **数据安全保护**：
  - 🔐 **访问控制**：基于 STS 临时凭证的细粒度权限控制
  - 🛡️ **数据加密**：COS 提供服务端加密，保护用户数据安全
  - 🚫 **防泄露机制**：临时凭证自动过期，避免长期密钥泄露风险
  - 📊 **访问审计**：完整的访问日志记录，可追溯数据操作历史

## 🛠️ 技术栈

### 后端
- **Rust** - 系统编程语言，高性能与内存安全
- **Axum** - 现代化异步 Web 框架
- **JWT** - 无状态认证令牌
- **bcrypt** - 密码安全哈希
- **WASM** - WebAssembly 媒体处理模块

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Tailwind CSS** - 实用优先的 CSS 框架
- **Axios** - HTTP 客户端库
- **Vue Router** - 官方路由管理器

### 基础设施
- **Nginx** - 高性能反向代理服务器
- **Vite** - 现代化前端构建工具
- **ESLint** - 代码质量检查工具
- **腾讯云 COS** - 对象存储服务，提供海量、安全、低成本的云存储

## 📦 项目结构

```
media-hub/
├── backend/                 # 后端服务
│   ├── media-server/       # 主服务器
│   │   ├── src/
│   │   │   ├── main.rs     # 服务入口
│   │   │   ├── auth.rs     # 认证中间件
│   │   │   ├── jwt.rs      # JWT 处理
│   │   │   └── user.rs     # 用户管理
│   │   └── Cargo.toml
│   └── media-wasm/         # WASM 模块
│       ├── src/
│       └── Cargo.toml
├── frontend/               # 前端应用
│   ├── src/
│   │   ├── api/           # API 客户端
│   │   ├── components/    # Vue 组件
│   │   ├── views/         # 页面视图
│   │   ├── router/        # 路由配置
│   │   └── utils/         # 工具函数
│   ├── package.json
│   └── vite.config.ts
├── nginx-media-hub.conf    # Nginx 配置
└── README.md
```

## 🚀 快速开始

### 环境要求

- **Rust** >= 1.70
- **Node.js** >= 18
- **Nginx** (用于生产部署)

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd media-hub

# 安装前端依赖
cd frontend
npm install

# 构建后端
cd ../backend/media-server
cargo build --release
```

### 开发环境

```bash
# 启动后端服务 (终端1)
cd backend/media-server
cargo run

# 启动前端开发服务器 (终端2)
cd frontend
npm run dev
```

### 生产部署

```bash
# 构建前端
cd frontend
npm run build

# 配置 Nginx
sudo cp nginx-media-hub.conf /etc/nginx/sites-available/media-hub
sudo ln -s /etc/nginx/sites-available/media-hub /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx

# 启动后端服务
cd backend/media-server
cargo run --release
```

## 🔧 配置

### 环境变量

```bash
# JWT 密钥 (生产环境必须设置)
export JWT_SECRET="your-secret-key-here"

# 服务端口
export PORT=8000

# 腾讯云 COS 配置 (文件上传功能必需)
export COS_SECRET_ID="your-cos-secret-id"
export COS_SECRET_KEY="your-cos-secret-key"
export COS_REGION="ap-beijing"
export COS_BUCKET="your-bucket-name"
```

### Nginx 配置

项目包含完整的 Nginx 配置文件 `nginx-media-hub.conf`，提供：
- 前端静态文件服务
- 后端 API 反向代理
- 静态资源缓存优化
- SPA 路由支持

## ☁️ 腾讯云 COS 集成

### 功能特性

- **安全上传**：使用 STS 临时凭证，避免长期密钥泄露风险
- **直传架构**：客户端直接上传到 COS，减轻服务器负载
- **进度监控**：实时显示上传进度和传输速度
- **文件验证**：上传前进行文件类型和大小验证
- **错误处理**：完善的错误提示和重试机制

### 数据安全保护

1. **访问控制**：基于 STS 临时凭证的细粒度权限控制
2. **数据加密**：COS 提供服务端加密，保护用户数据安全
3. **防泄露机制**：临时凭证自动过期，避免长期密钥泄露风险
4. **访问审计**：完整的访问日志记录，可追溯数据操作历史

### 配置说明

详细的 COS 配置指南请参考：[backend/media-server/COS_SETUP.md](backend/media-server/COS_SETUP.md)

## 🔐 认证系统

### 设计特点

- **HttpOnly Cookie**：token 存储在 HttpOnly Cookie 中，防止 XSS 攻击
- **双层存储**：Cookie 存储 token，localStorage 存储用户信息
- **智能验证**：本地快速检查 + 服务端准确验证
- **自动管理**：登录/登出状态自动同步，无需手动处理

### 认证流程

1. **注册**：用户名唯一性检查 → 密码哈希存储
2. **登录**：凭据验证 → JWT 生成 → HttpOnly Cookie 设置
3. **请求认证**：Cookie 自动发送 → 中间件验证 → 用户信息注入
4. **登出**：Cookie 清除 → 本地状态清理 → 页面重定向

## 🧪 开发

### 代码规范

```bash
# 前端代码检查
cd frontend
npm run lint
npm run lint:fix

# 后端代码格式化
cd backend/media-server
cargo fmt
cargo clippy
```

### 测试

```bash
# 后端测试
cd backend/media-server
cargo test

# 前端测试
cd frontend
npm run test
```

## 📋 开发状态与计划

### ✅ 已完成功能
- [x] **JWT 认证系统** - 完整的用户注册、登录、认证中间件
- [x] **前后端整合** - Vue 3 + Rust Axum 全栈架构
- [x] **Nginx 部署配置** - 生产环境反向代理配置
- [x] **现代化 UI 设计** - 统一的导航栏和响应式界面
- [x] **媒体文件管理** - 基础的媒体文件 CRUD 操作
- [x] **文件上传功能** - 支持多种媒体格式上传
- [x] **用户个人资料** - 用户信息管理和密码修改
- [x] **腾讯云 COS 集成** - 企业级对象存储，支持直传和安全访问控制

### 🚧 开发中功能
- [ ] **WASM 媒体处理模块** - 浏览器端媒体文件处理
- [ ] **高级媒体编辑** - 裁剪、滤镜、格式转换
- [ ] **媒体标签系统** - 分类和搜索功能
- [ ] **批量操作** - 多文件选择和批量处理

### 🔮 计划功能
- [ ] **媒体分享** - 公开链接和权限管理
- [ ] **API 文档** - 完整的 REST API 文档
- [ ] **Docker 部署** - 容器化部署方案
- [ ] **性能优化** - 缓存策略和加载优化
- [ ] **移动端适配** - PWA 支持和移动端优化

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License
