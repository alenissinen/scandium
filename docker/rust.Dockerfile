FROM rust:1.95-bookworm AS chef
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libcurl4-openssl-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY apps/ ./apps/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY apps/ ./apps/
COPY .sqlx/ ./.sqlx/
ENV SQLX_OFFLINE=true
RUN cargo build --release -p api -p consumer

# API
FROM debian:bookworm-slim AS api
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libcurl4 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/api ./api
EXPOSE 3001
CMD ["./api"]

# Consumer
FROM debian:bookworm-slim AS consumer
RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 libcurl4 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/consumer ./consumer
CMD ["./consumer"]