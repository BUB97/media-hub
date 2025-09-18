# 多阶段构建 - 构建阶段
FROM rust:1.87-slim as builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \ 
    pkg-config \                        
    libssl-dev \                        
    libpq-dev \                         
    && rm -rf /var/lib/apt/lists/*      
    
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update stable

# 设置工作目录
WORKDIR /app                            

# 复制 Cargo 文件
COPY backend/Cargo.toml backend/Cargo.toml ./  
COPY backend/media-server/Cargo.toml ./media-server/ 
COPY backend/media-wasm/Cargo.toml ./media-wasm/  

# 创建虚拟源文件以缓存依赖
RUN mkdir -p media-server/src media-wasm/src && \ 
    echo "fn main() {}" > media-server/src/main.rs && \  
    echo "fn main() {}" > media-wasm/src/lib.rs 

# 构建依赖（缓存层）
RUN cargo build --release --manifest-path media-server/Cargo.toml

# 删除虚拟源文件
RUN rm media-server/src/main.rs media-wasm/src/lib.rs 

# 复制实际源代码
COPY backend/ ./                       

# 构建应用程序
RUN cargo build --release --manifest-path media-server/Cargo.toml  

# 运行阶段
FROM debian:bookworm-slim               

# 安装运行时依赖
RUN apt-get update && apt-get install -y \  
    libpq5 \                            
    ca-certificates \                   
    curl \                             
    && rm -rf /var/lib/apt/lists/*      

# 创建非root用户
RUN useradd -r -s /bin/false appuser    

# 设置工作目录
WORKDIR /app                           

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/media-server ./media-server 

# 复制配置文件
COPY backend/media-server/.env.example ./.env  

# 创建日志目录
RUN mkdir -p logs && chown -R appuser:appuser /app  

# 切换到非root用户
USER appuser                            

# 暴露端口
EXPOSE 8000                             

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \  
    CMD curl -f http://localhost:8000/health || exit 1  

# 启动命令
CMD ["./media-server"]