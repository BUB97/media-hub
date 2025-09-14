# Media Hub 开发日志

## 2025年9月7日 - 系统整合与部署配置

### 今日完成的主要工作

#### 1. JWT 认证系统核心设计
- **安全认证架构**
  - HttpOnly Cookie 存储 JWT token，防止 XSS 攻击
  - bcrypt 密码哈希（cost=12）+ JWT 签名验证
  - 认证中间件统一处理 Cookie/Header 双重验证
  - SameSite=Strict 防护 CSRF 攻击

- **双层存储策略**
  - Cookie 存储 token（服务端管理，前端无法访问）
  - localStorage 存储用户信息（仅用于 UI 状态显示）
  - 实现安全性与用户体验的完美平衡

#### 2. 智能认证状态管理
- **认证检查机制**
  - `isAuthenticatedSync()`: 基于本地过期时间的快速检查
  - `isAuthenticated()`: 通过 API 验证 Cookie 有效性
  - 页面加载优先同步检查，后台异步验证确保准确性

- **自动状态同步**
  - API 拦截器监听 401 错误，自动清理过期状态
  - 登录/登出时协调更新 Cookie 和本地存储
  - 路由守卫基于认证状态自动重定向

#### 3. 统一架构设计
- **单域名架构**
  - Nginx 反向代理实现前后端统一访问
  - 解决跨域问题，简化 Cookie 配置

- **无状态认证**
  - JWT 包含完整用户信息，支持水平扩展
  - 服务端无需存储 session 状态

### 核心设计理念
- **安全优先**：HttpOnly Cookie + JWT 签名 + bcrypt 哈希的多层防护
- **用户体验**：本地缓存 + 异步验证平衡性能与准确性
- **架构简洁**：单域名统一访问，无状态水平扩展

### 系统设计成果
- ✅ 企业级 JWT 认证系统（HttpOnly Cookie + 双层存储）
- ✅ 智能认证状态管理（同步检查 + 异步验证）
- ✅ 统一架构设计（单域名 + 无状态扩展）

### 下一步计划
- [ ] WASM 媒体处理模块集成
- [ ] 媒体文件 CRUD 操作实现（集成数据库）
- [ ] 完整构建部署流程配置

---

## JWT 登录鉴权原理详解

### 后端 Token 管理机制

#### 1. JWT Token 创建流程
**创建时机**: 用户登录成功后，服务端验证用户名密码正确时创建JWT Token

```rust
// 位置: backend/media-server/src/main.rs (login endpoint)
// 用户登录成功后创建token
let token = create_token(user.id, &user.username)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

// 位置: backend/media-server/src/jwt.rs
pub fn create_token(user_id: i32, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, username.to_string());
    encode(&Header::default(), &claims, &ENCODING_KEY)
}

// Claims 结构体定义
pub struct Claims {
    pub sub: i32,        // 用户ID (subject)
    pub username: String, // 用户名
    pub exp: i64,        // 过期时间 (expiration)
    pub iat: i64,        // 签发时间 (issued at)
}

impl Claims {
    pub fn new(user_id: i32, username: String) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            username,
            iat: now.timestamp(),
            exp: (now + Duration::hours(24)).timestamp(), // 24小时过期
        }
    }
}
```

#### 2. Token 验证时机与机制
- **验证时机**: 每个需要认证的API请求都会触发验证
- **验证位置**: `auth_middleware` 中间件统一处理
- **验证流程**:
  ```rust
  // 位置: backend/media-server/src/auth.rs
  pub async fn auth_middleware<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
      // 1. 从Cookie中提取token
      let token = extract_token_from_cookie(&request)?;
      
      // 2. 验证token有效性和过期时间
      let claims = verify_token(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;
      
      // 3. 将用户信息注入请求上下文
      request.extensions_mut().insert(AuthUser {
          id: claims.sub,
          username: claims.username,
      });
      
      Ok(next.run(request).await)
  }
  ```

#### 3. 过期时间控制机制
- **设置过期时间**: Token创建时设置 `exp` 字段为当前时间 + 24小时
- **验证过期时间**: `jsonwebtoken` 库的 `Validation::default()` 自动检查 `exp` 字段
- **过期处理**: 过期token验证失败，返回401状态码

### 前端认证状态管理

#### 1. localStorage 过期时间存储机制
```typescript
// 位置: frontend/src/utils/auth.ts
export const setAuthData = (user: User, expiresAt: string) => {
  localStorage.setItem('user', JSON.stringify(user))
  localStorage.setItem('tokenExpiresAt', expiresAt) // 存储过期时间
}

// 快速同步检查（避免不必要的API调用）
export const isAuthenticatedSync = (): boolean => {
  const expiresAt = localStorage.getItem('tokenExpiresAt')
  if (!expiresAt) return false
  
  return new Date(expiresAt) > new Date() // 检查是否过期
}
```

#### 2. localStorage 过期时间的作用
- **性能优化**: 避免每次页面加载都发送API请求验证
- **用户体验**: 快速判断认证状态，减少页面闪烁
- **双重验证**: 本地快速检查 + 服务端权威验证的组合策略

#### 3. Cookie 自动传递机制
```typescript
// 位置: frontend/src/api/client.ts
const apiClient = axios.create({
  baseURL: '/api',
  withCredentials: true, // 关键配置：自动发送Cookie
})

// 登录时服务端设置HttpOnly Cookie
// 位置: backend/media-server/src/main.rs (login endpoint)
let cookie = Cookie::build(("auth_token", token))
    .http_only(true)     // 防止XSS攻击
    .same_site(SameSite::Strict) // 防止CSRF攻击
    .max_age(Duration::hours(24)) // Cookie过期时间
    .path("/")
    .build();
```

#### 4. 统一错误拦截与路由机制
```typescript
// 位置: frontend/src/api/client.ts
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // 1. 清除本地认证状态
      clearAuthData()
      
      // 2. 更新全局认证状态
      const authStore = useAuthStore()
      authStore.logout()
      
      // 3. 自动跳转到登录页面
      const router = useRouter()
      router.push('/login')
    }
    return Promise.reject(error)
  }
)
```

### JWT 认证流程完整时序图
```
用户登录 → 后端验证凭据 → 创建JWT(24h过期) → 设置HttpOnly Cookie
    ↓
前端存储用户信息+过期时间到localStorage → 页面跳转到Dashboard
    ↓
后续API请求 → Cookie自动发送 → 中间件验证JWT → 注入用户信息
    ↓
 Token过期 → 验证失败(401) → 前端拦截器清理状态 → 跳转登录页
```

### 安全设计要点
1. **HttpOnly Cookie**: 防止XSS攻击获取token
2. **SameSite=Strict**: 防止CSRF攻击
3. **双重过期控制**: JWT内置过期 + Cookie Max-Age
4. **无状态设计**: 服务端不存储session，支持水平扩展
5. **自动状态同步**: 401错误自动清理过期状态

---

## 技术架构总结

### 认证流程
**注册** → **登录** → **请求认证** → **登出**
- 用户名唯一性检查 + 密码哈希存储
- 凭据验证 + JWT 生成 + HttpOnly Cookie 设置
- Cookie 自动发送 + 中间件验证 + 用户信息注入
- Cookie 清除 + 本地状态清理 + 页面重定向

### 技术栈
```
前端: Vue 3 + TypeScript + axios
后端: Rust + Axum + JWT + bcrypt
代理: Nginx (静态文件 + API 代理)
```

---

**开发者备注**：今天主要完成了完整的 JWT 认证系统设计与实现，采用 HttpOnly Cookie 机制提升安全性，实现了前后端的无缝认证集成。系统现在具备了企业级的安全认证功能和良好的开发体验。