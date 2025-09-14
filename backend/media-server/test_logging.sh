#!/bin/bash

# Media Hub 完整功能测试脚本
# 包括认证、媒体操作、日志查询和监控功能

echo "🧪 测试 Media Hub 完整功能"
echo "================================================"

# 基础URL
BASE_URL="http://localhost:8000/api"

# Cookie 文件用于保存认证信息
COOKIE_FILE="cookies.txt"

# 清理之前的 Cookie 文件
rm -f "$COOKIE_FILE"

echo "\n1. 📊 检查服务器健康状态"
curl -s "$BASE_URL/health" | (jq . 2>/dev/null || cat)
echo

echo "\n2. 👤 用户注册"
REGISTER_RESPONSE=$(curl -s -c "$COOKIE_FILE" -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }')
echo "$REGISTER_RESPONSE" | (jq . 2>/dev/null || cat)
echo

echo "\n3. 🔐 用户登录"
LOGIN_RESPONSE=$(curl -s -c "$COOKIE_FILE" -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }')
echo "$LOGIN_RESPONSE" | (jq . 2>/dev/null || cat)
echo

echo "\n4. 👤 获取当前用户信息"
curl -s -b "$COOKIE_FILE" "$BASE_URL/auth/me" | (jq . 2>/dev/null || cat)
echo

echo "\n5. 📋 获取媒体列表（生成日志）"
curl -s -b "$COOKIE_FILE" "$BASE_URL/media" | (jq . 2>/dev/null || cat)
echo

echo "\n6. 🔍 搜索媒体（生成更多日志）"
curl -s -b "$COOKIE_FILE" "$BASE_URL/media/search?q=视频" | (jq . 2>/dev/null || cat)
echo

echo "\n7. ➕ 创建新媒体（生成日志）"
CREATE_RESPONSE=$(curl -s -b "$COOKIE_FILE" -X POST "$BASE_URL/media" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "测试视频",
    "description": "用于测试日志功能的视频",
    "file_path": "/test/video.mp4"
  }')
echo "$CREATE_RESPONSE" | (jq . 2>/dev/null || cat)
echo

echo "\n8. 📊 查看监控指标"
curl -s -b "$COOKIE_FILE" "$BASE_URL/metrics" | (jq . 2>/dev/null || cat)
echo

echo "\n9. 📝 查询所有日志记录"
curl -s -b "$COOKIE_FILE" "$BASE_URL/logs" | (jq . 2>/dev/null || cat)
echo

echo "\n10. 🔍 按日志级别查询（INFO）"
curl -s -b "$COOKIE_FILE" "$BASE_URL/logs?level=info" | (jq . 2>/dev/null || cat)
echo

echo "\n11. 📊 限制返回数量（最多5条）"
curl -s -b "$COOKIE_FILE" "$BASE_URL/logs?limit=5" | (jq . 2>/dev/null || cat)
echo

echo "\n12. 🕒 按时间范围查询（最近1小时）"
# macOS 和 Linux 兼容的日期命令
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    START_TIME=$(date -u -v-1H +"%Y-%m-%dT%H:%M:%SZ")
else
    # Linux
    START_TIME=$(date -u -d '1 hour ago' +"%Y-%m-%dT%H:%M:%SZ")
fi
echo "查询时间范围: $START_TIME 到现在"
curl -s -b "$COOKIE_FILE" "$BASE_URL/logs?start_time=$START_TIME" | (jq . 2>/dev/null || cat)
echo

echo "\n13. 👤 按用户ID查询日志"
curl -s -b "$COOKIE_FILE" "$BASE_URL/logs?user_id=demo_user_123" | (jq . 2>/dev/null || cat)
echo

echo "\n14. 🔄 用户登出"
LOGOUT_RESPONSE=$(curl -s -b "$COOKIE_FILE" -c "$COOKIE_FILE" -X POST "$BASE_URL/auth/logout")
echo "$LOGOUT_RESPONSE" | (jq . 2>/dev/null || cat)
echo

echo "\n15. 🚫 测试登出后访问受保护端点（应该失败）"
echo "尝试访问媒体列表（应该返回401错误）:"
curl -s -b "$COOKIE_FILE" "$BASE_URL/media" | (jq . 2>/dev/null || cat)
echo

# 清理 Cookie 文件
rm -f "$COOKIE_FILE"

echo "\n✅ 测试完成！"
echo "\n💡 提示："
echo "  - 所有受保护的端点都需要先登录获取认证Cookie"
echo "  - 日志文件保存在 logs/ 目录下"
echo "  - 可以使用 RUST_LOG=debug 环境变量调整日志级别"
echo "  - 监控指标包括错误数、警告数等统计信息"
echo "  - 支持按用户ID、时间范围、日志级别等条件查询日志"
echo "  - 使用 jq 工具格式化JSON输出（如果未安装请运行: brew install jq）"