FROM amd64/rust:1.88 AS builder
#FROM rust:1.88 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY . .
RUN cargo build --release

#FROM debian:bookworm-slim
FROM amd64/debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/api_rust_sf /app/api_rust_sf
EXPOSE 8085

CMD ["./api_rust_sf"]
