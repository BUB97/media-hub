#!/bin/bash

# 日志、错误监控和查询功能测试脚本

echo "🧪 测试 Media Hub 日志、错误监控和查询功能"
echo "================================================"

# 基础URL
BASE_URL="http://localhost:3000"

echo "\n1. 📊 检查服务器健康状态"
curl -s "$BASE_URL/health"
echo

echo "\n2. 📋 获取媒体列表（生成日志）"
curl -s "$BASE_URL/api/media"
echo

echo "\n3. 🔍 搜索媒体（生成更多日志）"
curl -s "$BASE_URL/api/media/search?q=视频"
echo

echo "\n4. ➕ 创建新媒体（生成日志）"
curl -s -X POST "$BASE_URL/api/media" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "测试视频",
    "description": "用于测试日志功能的视频",
    "file_path": "/test/video.mp4"
  }'
echo

echo "\n5. 📊 查看监控指标"
curl -s "$BASE_URL/api/metrics"
echo

echo "\n6. 📝 查询所有日志记录"
curl -s "$BASE_URL/api/logs"
echo

echo "\n7. 🔍 按日志级别查询（INFO）"
curl -s "$BASE_URL/api/logs?level=info"
echo

echo "\n8. 📊 限制返回数量（最多5条）"
curl -s "$BASE_URL/api/logs?limit=5"
echo

echo "\n9. 🕒 按时间范围查询（最近1小时）"
START_TIME=$(date -u -v-1H +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date -u -d '1 hour ago' +"%Y-%m-%dT%H:%M:%SZ")
echo "查询时间范围: $START_TIME 到现在"
curl -s "$BASE_URL/api/logs?start_time=$START_TIME"
echo

echo "\n✅ 测试完成！"
echo "\n💡 提示："
echo "  - 日志文件保存在 logs/ 目录下"
echo "  - 可以使用 RUST_LOG=debug 环境变量调整日志级别"
echo "  - 监控指标包括错误数、警告数等统计信息"
echo "  - 支持按用户ID、时间范围、日志级别等条件查询日志"