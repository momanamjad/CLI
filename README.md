# github-cli

A production-grade Rust CLI tool and WebSocket PTY server that powers a real browser-based terminal — the same architecture used by GitHub Codespaces, Replit, and VS Code Server.

![Rust](https://img.shields.io/badge/Rust-1.85-orange?logo=rust)
![Railway](https://img.shields.io/badge/Deployed-Railway-purple?logo=railway)
![License](https://img.shields.io/badge/license-MIT-blue)

## 🌐 Live Demo
**[github-kappa-two.vercel.app/terminal](https://github-kappa-two.vercel.app/terminal)**

Open the link, click the terminal, and type real bash commands like `git log`, `ls`, `cat package.json`.

## 🏗️ Architecture

```text
+-------------------+        WebSocket         +---------------------+
| Browser / React   |  <-------------------->  | Rust axum Server    |
| (xterm.js)        |       (JSON/Binary)      | (portable-pty)      |
+-------------------+                          +---------------------+
                                                          |
                                                          v
                                                 +-------------------+
                                                 | Real Shell        |
                                                 | (bash/ConPTY)     |
                                                 +-------------------+
```

## ✨ Features
- Real PTY terminal in the browser (not a fake shell)
- Live dashboard: LOC counter, file distribution, dependency list, git status
- REST API for project intelligence
- Copy/paste support (Ctrl+Shift+C/V, right-click)
- Terminal resize support
- Cross-platform: ConPTY on Windows, bash on Linux
- Auto-clones target project on startup

## 🛠️ Tech Stack

| Technology      | Purpose                                    |
|-----------------|--------------------------------------------|
| **Rust**        | Core server language                       |
| **axum**        | HTTP and WebSocket routing framework       |
| **tokio**       | Asynchronous runtime                       |
| **portable-pty**| Pseudo-terminal interface (PTY)            |
| **serde**       | JSON serialization and deserialization     |
| **xterm.js**    | Web-based terminal emulator                |
| **React 19**    | Frontend UI rendering                      |

## 📡 API Endpoints

| Method | Endpoint      | Description                                      |
|--------|---------------|--------------------------------------------------|
| GET    | `/stats`      | Returns project file stats and LOC counts        |
| GET    | `/deps`       | Lists `package.json` dependencies if present     |
| GET    | `/ls`         | Lists files in directory (accepts `?path=...`)   |
| GET    | `/git/status` | Returns current `git status` output              |
| GET    | `/git/log`    | Returns `git log` output (accepts `?n=...`)      |
| GET    | `/git/branch` | Returns current branches                         |
| POST   | `/search`     | Searches for a query string across files         |
| WS     | `/ws`         | WebSocket connection for the real-time PTY shell |

## 🔧 How It Works

A Pseudo-Terminal (PTY) allows a program to emulate a real terminal device. Normally, terminal programs like `bash` expect to be attached to an actual terminal emulator window (like iTerm or Windows Terminal). By using `portable-pty`, our Rust server creates a "fake" terminal device in memory, allowing `bash` to run exactly as if a human was sitting at a keyboard, complete with colors, formatting, and interactive program support (like `vim` or `top`).

The connection between the browser and this PTY is bridged over WebSockets. When you type a character in the browser using `xterm.js`, it sends a WebSocket message to the Rust server. Rust feeds that keystroke directly into the PTY's standard input. When the shell produces output (like text, ANSI colors, or cursor movements), the PTY captures it, Rust reads it, and blasts those raw bytes down the WebSocket to `xterm.js` for rendering. 

Rust was chosen because bridging binary streams between a PTY and a WebSocket requires extremely low-latency, memory-safe, and highly concurrent execution. `tokio` effortlessly manages the asynchronous streams, while Rust's strict memory guarantees prevent the kind of buffer overflow and resource leak vulnerabilities common when interfacing directly with OS-level pipes and processes in C or C++.

## 🚀 Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/momanamjad/CLI.git github-cli
   cd github-cli
   ```

2. **Start the backend server:**
   ```bash
   cargo run
   ```
   *The server starts on port 3001.*

3. **Verify it's running:**
   Open `http://localhost:3001/stats` in your browser.

4. **Start the React frontend** (in your separate frontend directory):
   ```bash
   npm run dev
   ```

## 🏛️ Same Architecture As
- GitHub Codespaces
- Replit
- VS Code Server
- Railway shell tabs

## 👤 Author
Moman Amjad — [github.com/momanamjad](https://github.com/momanamjad)
