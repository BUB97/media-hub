# 腾讯云COS配置指南

## 🔧 环境变量配置

要使用腾讯云COS功能，您需要配置以下环境变量：

### 必需的环境变量

```bash
# 腾讯云SecretId（从腾讯云控制台获取）
export COS_SECRET_ID="your_actual_secret_id"

# 腾讯云SecretKey（从腾讯云控制台获取）
export COS_SECRET_KEY="your_actual_secret_key"

# COS存储桶所在区域
export COS_REGION="ap-beijing"  # 或其他区域如 ap-shanghai, ap-guangzhou
```

### 可选的环境变量

```bash
# COS存储桶名称（如果需要）
export COS_BUCKET="your-bucket-name"

# 上传文件前缀路径
export COS_UPLOAD_PREFIX="media/"
```

## 🚀 启动服务器

### 方法1：临时设置环境变量

```bash
# 在启动命令前设置环境变量
COS_SECRET_ID="your_secret_id" COS_SECRET_KEY="your_secret_key" COS_REGION="ap-beijing" cargo run
```

### 方法2：使用.env文件（推荐）

1. 在项目根目录创建 `.env` 文件：

```bash
# .env 文件内容
COS_SECRET_ID=your_actual_secret_id
COS_SECRET_KEY=your_actual_secret_key
COS_REGION=ap-beijing
```

2. 安装dotenv支持（如果还没有）：

```bash
cargo add dotenv
```

3. 在main.rs中加载.env文件：

```rust
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // 加载.env文件
    dotenv().ok();
    
    // 其他初始化代码...
}
```

### 方法3：系统环境变量

将环境变量添加到您的shell配置文件中（如 `~/.zshrc` 或 `~/.bashrc`）：

```bash
echo 'export COS_SECRET_ID="your_secret_id"' >> ~/.zshrc
echo 'export COS_SECRET_KEY="your_secret_key"' >> ~/.zshrc
echo 'export COS_REGION="ap-beijing"' >> ~/.zshrc
source ~/.zshrc
```

## 🔑 获取腾讯云凭证

1. 登录 [腾讯云控制台](https://console.cloud.tencent.com/)
2. 进入 [访问管理 > API密钥管理](https://console.cloud.tencent.com/cam/capi)
3. 创建新的API密钥或使用现有密钥
4. 复制 SecretId 和 SecretKey

## 🛡️ 安全注意事项

- **永远不要**将真实的SecretId和SecretKey提交到版本控制系统
- 将 `.env` 文件添加到 `.gitignore` 中
- 在生产环境中使用更安全的密钥管理方案
- 定期轮换API密钥

## 🧪 测试配置

配置完成后，可以使用以下命令测试：

```bash
# 启动服务器
cargo run

# 在另一个终端测试STS功能
./test_cos_handlers.sh
```

## 🔍 故障排除

### 常见错误

1. **"未配置腾讯云SecretId"**
   - 检查环境变量是否正确设置
   - 确保在启动服务器前设置了环境变量

2. **"AuthFailure.SecretIdNotFound"**
   - 检查SecretId是否正确
   - 确保API密钥未被删除或禁用

3. **"SignatureDoesNotMatch"**
   - 检查SecretKey是否正确
   - 确保没有多余的空格或特殊字符

### 调试步骤

1. 检查环境变量：
```bash
echo $COS_SECRET_ID
echo $COS_SECRET_KEY
echo $COS_REGION
```

2. 查看服务器日志中的详细错误信息

3. 使用腾讯云CLI工具验证凭证：
```bash
tccli configure set secretId your_secret_id
tccli configure set secretKey your_secret_key
tccli configure set region ap-beijing
tccli sts GetFederationToken
```

## 📚 相关文档

- [腾讯云COS官方文档](https://cloud.tencent.com/document/product/436)
- [腾讯云STS官方文档](https://cloud.tencent.com/document/product/598/33416)
- [cos-rust-sdk文档](https://github.com/tencentyun/cos-rust-sdk)