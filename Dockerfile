FROM debian:bookworm-slim

# Install full dev environment
RUN apt-get update && apt-get install -y \
    git \
    curl \
    bash \
    ca-certificates \
    openssl \
    wget \
    vim \
    tree \
    htop \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Configure git
RUN git config --global user.name "github-cli" \
    && git config --global user.email "cli@github-clone.dev" \
    && git config --global init.defaultBranch main

# Clone the GitHub clone project into the container
COPY --from=builder /app/target/release/github-cli /usr/local/bin/github-cli

WORKDIR /workspace

ENV SERVER_ONLY=true
ENV PORT=3001
ENV PROJECT_PATH=/workspace

CMD ["github-cli"]