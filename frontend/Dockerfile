# 多阶段构建 - 构建阶段
FROM node:24-alpine as builder

# 设置工作目录
WORKDIR /app

# 复制 package 文件
COPY frontend/package*.json ./

# 安装所有依赖（包括devDependencies用于构建，跳过prepare脚本以避免husky问题）
RUN npm ci --ignore-scripts

# 复制源代码
COPY frontend/ ./

# 构建应用
RUN npm run build

# 生产阶段 - Nginx
FROM nginx:alpine

# 安装 curl 用于健康检查
RUN apk add --no-cache curl

# 复制前端容器的 server 级 Nginx 配置
COPY frontend/docker-nginx.conf /etc/nginx/conf.d/default.conf

# 从构建阶段复制构建产物
COPY --from=builder /app/dist /usr/share/nginx/html

# 不复制主机用的 nginx-media-hub.conf，避免与容器内配置冲突

# 暴露端口
EXPOSE 80

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# 启动 Nginx
CMD ["nginx", "-g", "daemon off;"]