# 多阶段构建 - 构建阶段
FROM rust:1.70-slim as builder          # 基础镜像：Rust 1.70精简版，命名为builder阶段

# 安装构建依赖
RUN apt-get update && apt-get install -y \  # 更新包列表并安装依赖
    pkg-config \                        # 包配置工具，用于编译时查找库
    libssl-dev \                        # OpenSSL开发库，用于HTTPS支持
    libpq-dev \                         # PostgreSQL开发库，用于数据库连接
    && rm -rf /var/lib/apt/lists/*      # 清理包缓存，减小镜像大小

# 设置工作目录
WORKDIR /app                            # 设置容器内工作目录为/app

# 复制 Cargo 文件
COPY backend/Cargo.toml backend/cargo.toml ./  # 复制工作空间Cargo配置
COPY backend/media-server/Cargo.toml ./media-server/  # 复制media-server项目配置
COPY backend/media-wasm/Cargo.toml ./media-wasm/      # 复制WASM项目配置

# 创建虚拟源文件以缓存依赖
RUN mkdir -p media-server/src media-wasm/src && \  # 创建源代码目录
    echo "fn main() {}" > media-server/src/main.rs && \  # 创建虚拟main.rs
    echo "fn main() {}" > media-wasm/src/lib.rs     # 创建虚拟lib.rs

# 构建依赖（缓存层）
RUN cargo build --release --manifest-path media-server/Cargo.toml  # 仅构建依赖，利用Docker层缓存

# 删除虚拟源文件
RUN rm media-server/src/main.rs media-wasm/src/lib.rs  # 删除虚拟文件

# 复制实际源代码
COPY backend/ ./                        # 复制所有后端源代码

# 构建应用程序
RUN cargo build --release --manifest-path media-server/Cargo.toml  # 构建实际应用

# 运行阶段
FROM debian:bookworm-slim               # 基础镜像：Debian精简版，用于运行

# 安装运行时依赖
RUN apt-get update && apt-get install -y \  # 安装运行时必需的库
    libpq5 \                            # PostgreSQL客户端库
    ca-certificates \                   # CA证书，用于HTTPS连接
    curl \                              # curl工具，用于健康检查
    && rm -rf /var/lib/apt/lists/*      # 清理包缓存

# 创建非root用户
RUN useradd -r -s /bin/false appuser    # 创建系统用户appuser，提升安全性

# 设置工作目录
WORKDIR /app                            # 设置工作目录

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/media-server ./media-server  # 从builder阶段复制编译好的二进制

# 复制配置文件
COPY backend/media-server/.env.example ./.env  # 复制环境配置示例

# 创建日志目录
RUN mkdir -p logs && chown -R appuser:appuser /app  # 创建日志目录并设置权限

# 切换到非root用户
USER appuser                            # 切换到非特权用户运行

# 暴露端口
EXPOSE 8000                             # 暴露8000端口

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \  # 健康检查配置
    CMD curl -f http://localhost:8000/health || exit 1  # 健康检查命令

# 启动命令
CMD ["./media-server"]                  # 容器启动时执行的命令