# GitHub Clone Terminal CLI (Rust)

A fast, native terminal client built in Rust for bidirectional repository file tree synchronization, repository statistics, dependency mapping, and secure secrets vault management.

---

## 🚀 Commands

### Authentication
* `login <email> <password>` — Log in and save authentication token to global configuration.

### Remote Repositories
* `remote-create <name> [description]` — Create a new repository on the web server and remote-link it locally.
* `remote-link <repository_id>` — Link local directory to a server repository.
* `remote-push` — Scan local files (respects ignores) and push update tree to the server.
* `remote-pull` — Download and pull web repository changes to the local folder.

### Secrets Vault Management
* `secret-set <key> <value>` — Encrypt and upload a secret key to the repository.
* `secret-list` — List all registered secrets (names only).
* `secret-delete <key>` — Delete a secret from the vault.

### Local Utilities
* `stats` — Displays lines of code, file counts, and language breakdown.
* `deps` — Lists dependency structures in a formatted table.
* `git-status` / `git-log` — Show local git status and commit stream logs.

---

## 🛠️ Build & Development
Build the optimized executable binary:
```bash
cargo build --release
```
The compiled output is created at `target/release/github-cli.exe`.
