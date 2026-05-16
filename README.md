# GitHub CLI 🚀

A powerful, production-ready Rust terminal and backend server designed for interacting with modern web projects (React, Vite, Tailwind, etc.). This tool combines a feature-rich CLI with a robust REST API and WebSocket-based PTY for remote terminal access.

## 🌟 Features

- **Project Insights**: Get instant statistics and dependency analysis of your web projects.
- **Git Integration**: Built-in commands for `git status`, `git log`, and `git branch`.
- **Advanced File Navigation**: Explore your project with `ls`, `cat`, `find`, and `search` functionality.
- **Interactive CLI**: A sleek, colored terminal interface for local development.
- **REST API Backend**: Exposes project data via standard HTTP endpoints (Stats, Deps, Search, Git).
- **Live Terminal (WebSocket PTY)**: Real-time terminal access via WebSockets, allowing you to run shell commands through the backend.
- **Cloud-Ready**: Optimized for deployment on platforms like Railway with `SERVER_ONLY` mode and Docker support.

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (2024 Edition)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Terminal PTY**: [Portable-PTY](https://github.com/wez/wezterm/tree/main/portable-pty)
- **Serialization**: [Serde](https://serde.rs/) & [TOML](https://github.com/toml-rs/toml)
- **Styling**: [Colored](https://github.com/mackwic/colored) & [PrettyTable](https://github.com/phstc/rust-prettytable)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Configuration

Create or modify `config.toml` in the project root to point to your target project:

```toml
project_path = 'C:\path\to\your\project'
```

### Running Locally

To start both the CLI and the backend server:

```bash
cargo run
```

### Server-Only Mode (Cloud Deployment)

If you only want to run the backend server (e.g., for Railway or Docker):

```bash
# Windows
$env:SERVER_ONLY="1"; cargo run

# Linux/macOS
SERVER_ONLY=1 cargo run
```

## 🌐 API Endpoints

The backend server runs by default on `http://localhost:3001` (or the port specified by the `PORT` environment variable).

| Endpoint | Method | Description |
| :--- | :--- | :--- |
| `/` | `GET` | Health check |
| `/stats` | `GET` | Get project statistics |
| `/deps` | `GET` | Get project dependencies |
| `/ls?path=.` | `GET` | List files in a directory |
| `/search` | `POST` | Search for keywords in the project |
| `/git/status` | `GET` | Get git status |
| `/git/log?n=5` | `GET` | Get recent git logs |
| `/git/branch` | `GET` | Get current git branch |
| `/ws` | `WS` | Real-time terminal (PTY) connection |

## 📦 Docker Support

You can build and run the project using Docker:

```bash
docker build -t github-cli .
docker run -p 3001:3001 -e PROJECT_PATH=/app/my-project github-cli
```

## 📜 License

This project was built as part of a Rust learning journey. Feel free to explore, modify, and learn!
