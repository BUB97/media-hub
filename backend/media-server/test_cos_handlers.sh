#!/bin/bash

# COS Handlers 测试脚本
# 测试腾讯云COS相关的API端点

echo "🧪 开始测试 COS Handlers..."
echo "======================================"

# 服务器地址
BASE_URL="http://localhost:8000"

# 存储Cookie的临时文件
COOKIE_FILE="/tmp/cos_test_cookies.txt"

# 清理函数
cleanup() {
    echo "\n🧹 清理临时文件..."
    rm -f "$COOKIE_FILE"
    echo "✅ 测试完成！"
}

# 设置退出时清理
trap cleanup EXIT

echo "\n1️⃣ 健康检查"
curl -s "$BASE_URL/api/health" | (jq . 2>/dev/null || cat)

echo "\n\n2️⃣ 用户注册"
curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "cosuser",
    "email": "cosuser@example.com",
    "password": "password123"
  }' | (jq . 2>/dev/null || cat)

echo "\n\n3️⃣ 用户登录 (获取认证Cookie)"
curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -c "$COOKIE_FILE" \
  -d '{
    "username": "cosuser",
    "password": "password123"
  }' | (jq . 2>/dev/null || cat)

echo "\n\n4️⃣ 获取COS配置信息"
curl -s -X GET "$BASE_URL/api/cos/config" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n5️⃣ 获取STS临时凭证 (默认参数)"
curl -s -X GET "$BASE_URL/api/cos/sts" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n6️⃣ 获取STS临时凭证 (自定义持续时间)"
curl -s -X GET "$BASE_URL/api/cos/sts?duration_seconds=3600" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n7️⃣ 获取STS临时凭证 (自定义Policy)"
curl -s -X GET "$BASE_URL/api/cos/sts?duration_seconds=1800&policy=%7B%22version%22%3A%222.0%22%2C%22statement%22%3A%5B%7B%22effect%22%3A%22allow%22%2C%22action%22%3A%5B%22cos%3AGetObject%22%5D%2C%22resource%22%3A%5B%22*%22%5D%7D%5D%7D" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n8️⃣ 验证文件上传参数"
curl -s -X POST "$BASE_URL/api/cos/validate" \
  -H "Content-Type: application/json" \
  -b "$COOKIE_FILE" \
  -d '{
    "filename": "test-video.mp4",
    "file_size": 10485760,
    "content_type": "video/mp4"
  }' | (jq . 2>/dev/null || cat)

echo "\n\n9️⃣ 验证文件上传参数 (无效文件名)"
curl -s -X POST "$BASE_URL/api/cos/validate" \
  -H "Content-Type: application/json" \
  -b "$COOKIE_FILE" \
  -d '{
    "filename": "../../../etc/passwd",
    "file_size": 1024,
    "content_type": "text/plain"
  }' | (jq . 2>/dev/null || cat)

echo "\n\n🔟 验证文件上传参数 (文件过大)"
curl -s -X POST "$BASE_URL/api/cos/validate" \
  -H "Content-Type: application/json" \
  -b "$COOKIE_FILE" \
  -d '{
    "filename": "huge-file.zip",
    "file_size": 5368709120,
    "content_type": "application/zip"
  }' | (jq . 2>/dev/null || cat)

echo "\n\n1️⃣1️⃣ 用户登出"
curl -s -X POST "$BASE_URL/api/auth/logout" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n1️⃣2️⃣ 登出后尝试访问COS端点 (应该返回401)"
curl -s -X GET "$BASE_URL/api/cos/sts" \
  -b "$COOKIE_FILE" | (jq . 2>/dev/null || cat)

echo "\n\n======================================"
echo "📝 测试说明:"
echo "  - 步骤 1-3: 基础认证流程"
echo "  - 步骤 4: 获取COS配置信息"
echo "  - 步骤 5-7: 测试STS临时凭证获取 (不同参数)"
echo "  - 步骤 8-10: 测试文件上传验证 (正常/异常情况)"
echo "  - 步骤 11-12: 登出和权限验证"
echo "\n⚠️  注意: 如果没有配置COS环境变量，STS相关请求会返回错误"
echo "    需要设置: COS_SECRET_ID, COS_SECRET_KEY, COS_REGION"