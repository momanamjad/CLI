FROM rust:1.86-slim AS builder
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
    wget \
    vim \
    tree \
    iproute2 \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN git config --global user.name "github-cli" \
    && git config --global user.email "cli@github-clone.dev" \
    && git config --global init.defaultBranch main

COPY --from=builder /app/target/release/github-cli /usr/local/bin/github-cli
COPY start.sh /start.sh
RUN chmod +x /start.sh

WORKDIR /workspace
RUN mkdir -p /workspace/project

ENV SERVER_ONLY=true
ENV PORT=3001
ENV PROJECT_PATH=/workspace/project

CMD ["/start.sh"]