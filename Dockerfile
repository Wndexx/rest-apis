# 构建阶段
FROM clux/muslrust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
# 构建空 noop 二进制用于缓存依赖
RUN mkdir src \
    && echo "fn main() {}" > src/noop.rs \
    && echo '[[bin]]' >> Cargo.toml \
    && echo 'name = "dep-builder"' >> Cargo.toml \
    && echo 'path = "src/noop.rs"' >> Cargo.toml \
    && cargo build --release --target x86_64-unknown-linux-musl

# 拷贝源码并构建实际项目
COPY src/ src/
RUN cargo build --release --target x86_64-unknown-linux-musl

# 运行阶段：仅包含最终二进制与证书
FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-apis /app/rust-apis
EXPOSE 3000
ENTRYPOINT ["/app/rust-apis"]
