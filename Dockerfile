FROM rust:1.85-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    git \
    curl \
    bash \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/github-cli /usr/local/bin/github-cli
ENV PORT=3001
ENV PROJECT_PATH=.
ENV SERVER_ONLY=true
CMD ["github-cli"]
