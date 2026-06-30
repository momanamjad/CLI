use std::io::{self, Write};
use colored::*;
use crate::commands::{help, about, ls, cat, find, pwd, open, stats, deps, search, recent, git};
use crate::config::Config;
use crate::server;
use walkdir::WalkDir;

pub struct CommandHandler {
    config: Config,
}

pub async fn run() {
    let mut handler = CommandHandler::new();
    handler.run().await;
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }

    pub async fn run(&mut self) {
        println!("{}", "╔══════════════════════════════════╗".bright_green());
        println!("║       {}      ║", "GitHub Clone CLI v0.2".bold().white());
        println!("║  {} ║", "Production-ready Rust Terminal".dimmed());
        println!("{}", "╚══════════════════════════════════╝".bright_green());
        println!("Project path: {}\n", self.config.project_path.yellow());

        loop {
            print!("{}", "github-cli> ".bright_blue().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF reached, exit loop
                Ok(_) => {},
                Err(_) => continue,
            }
            let input = input.trim();
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "help" => println!("{}", help::run()),
                "serve" => server::start_server().await,
                "about" => println!("{}", about::run()),
                "pwd" => println!("{}", pwd::run()),
                "ls" => {
                    let path = if parts.len() > 1 { parts[1] } else { "." };
                    println!("{}", ls::run(path));
                }
                "cat" => {
                    if parts.len() < 2 {
                        println!("Usage: cat <filepath>");
                    } else {
                        println!("{}", cat::run(parts[1]));
                    }
                }
                "find" => {
                    if parts.len() < 3 {
                        println!("Usage: find <path> <filename>");
                    } else {
                        println!("{}", find::run(parts[1], parts[2]));
                    }
                }
                "open" => {
                    let path = if parts.len() > 1 { parts[1] } else { "." };
                    println!("{}", open::run(path));
                }
                "stats" => println!("{}", stats::run(&self.config.project_path)),
                "deps" => println!("{}", deps::run(&self.config.project_path)),
                "search" => {
                    if parts.len() < 2 {
                        println!("Usage: search <keyword>");
                    } else {
                        println!("{}", search::run(&self.config.project_path, parts[1]));
                    }
                }
                "recent" => println!("{}", recent::run(&self.config.project_path)),
                "git-status" => println!("{}", git::status(&self.config.project_path)),
                "git-log" => {
                    let n = if parts.len() > 1 { parts[1] } else { "5" };
                    println!("{}", git::log(&self.config.project_path, n));
                }
                "git-branch" => println!("{}", git::branch(&self.config.project_path)),
                "login" => {
                    if parts.len() < 3 {
                        println!("Usage: login <email> <password>");
                    } else {
                        let email = parts[1];
                        let password = parts[2];
                        let mut temp_config = self.config.clone();
                        match handle_login(email, password, &mut temp_config).await {
                            Ok(_token) => {
                                self.config = temp_config;
                                println!("{}", "Login successful! Token saved.".green());
                            }
                            Err(err) => {
                                println!("{} {}", "Login failed:".red(), err);
                            }
                        }
                    }
                }
                "remote-create" => {
                    if parts.len() < 2 {
                        println!("Usage: remote-create <repo_name> [description]");
                    } else {
                        let name = parts[1];
                        let desc = if parts.len() > 2 { parts[2] } else { "" };
                        match handle_remote_create(name, desc, &self.config).await {
                            Ok(repo_id) => {
                                println!("{} {}", "Repository created successfully! ID:".green(), repo_id);
                            }
                            Err(err) => {
                                println!("{} {}", "Failed to create remote repository:".red(), err);
                            }
                        }
                    }
                }
                "remote-link" => {
                    if parts.len() < 2 {
                        println!("Usage: remote-link <repo_id>");
                    } else {
                        let id = parts[1];
                        match handle_remote_link(id, &self.config).await {
                            Ok(name) => {
                                println!("{} {}", "Repository linked successfully! Name:".green(), name);
                            }
                            Err(err) => {
                                println!("{} {}", "Failed to link repository:".red(), err);
                            }
                        }
                    }
                }
                "remote-pull" => {
                    match handle_remote_pull(&self.config).await {
                        Ok(_) => {
                            println!("{}", "Repository files pulled successfully!".green());
                        }
                        Err(err) => {
                            println!("{} {}", "Failed to pull files:".red(), err);
                        }
                    }
                }
                "remote-push" => {
                    match handle_remote_push(&self.config).await {
                        Ok(_) => {
                            println!("{}", "Repository files pushed successfully!".green());
                        }
                        Err(err) => {
                            println!("{} {}", "Failed to push files:".red(), err);
                        }
                    }
                }
                "secret-set" => {
                    if parts.len() < 3 {
                        println!("Usage: secret-set <NAME> <VALUE>");
                    } else {
                        let name = parts[1];
                        let value = parts[2..].join(" ");
                        let value_clean = value.trim_matches(|c| c == '"' || c == '\'');
                        match handle_secret_set(name, value_clean, &self.config).await {
                            Ok(_) => println!("{}", "Secret saved successfully!".green()),
                            Err(err) => println!("{} {}", "Failed to save secret:".red(), err),
                        }
                    }
                }
                "secret-list" => {
                    match handle_secret_list(&self.config).await {
                        Ok(secrets) => {
                            if secrets.is_empty() {
                                println!("No secrets defined for this repository.");
                            } else {
                                println!("\n{}", "=== Repository Secrets ===".yellow().bold());
                                for secret in secrets {
                                    println!("  - {} (created: {})", secret.name.green(), secret.created_at);
                                }
                            }
                        }
                        Err(err) => println!("{} {}", "Failed to list secrets:".red(), err),
                    }
                }
                "secret-delete" => {
                    if parts.len() < 2 {
                        println!("Usage: secret-delete <NAME>");
                    } else {
                        let name = parts[1];
                        match handle_secret_delete(name, &self.config).await {
                            Ok(_) => println!("{}", "Secret deleted successfully!".green()),
                            Err(err) => println!("{} {}", "Failed to delete secret:".red(), err),
                        }
                    }
                }
                "remote-pull-wiki" => {
                    match handle_remote_pull_wiki(&self.config).await {
                        Ok(_) => println!("{}", "Repository Wiki pulled successfully!".green()),
                        Err(err) => println!("{} {}", "Failed to pull Wiki:".red(), err),
                    }
                }
                "remote-push-wiki" => {
                    match handle_remote_push_wiki(&self.config).await {
                        Ok(_) => println!("{}", "Repository Wiki pushed successfully!".green()),
                        Err(err) => println!("{} {}", "Failed to push Wiki:".red(), err),
                    }
                }
                "exit" => {
                    println!("{}", "Goodbye!".bright_red());
                    break;
                }
                _ => {
                    println!("'{}' is not a recognized command. Type 'help'.", parts[0].red());
                }
            }
            println!(); // Extra newline for readability
        }
    }
}

async fn handle_login(email: &str, password: &str, config: &mut Config) -> Result<String, String> {
    let api_url = resolve_api_url(config).await;
    let client = reqwest::Client::new();
    
    let res = client.post(&format!("{}/auth/login", api_url))
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Login failed: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid JSON response: {}", e))?;

    let token = json_resp["data"]["accessToken"].as_str()
        .ok_or("Access token not found in login response.")?
        .to_string();

    // Save token to config.toml
    config.token = Some(token.clone());
    Config::save(config).map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(token)
}

async fn handle_remote_create(name: &str, desc: &str, config: &Config) -> Result<String, String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let api_url = resolve_api_url(config).await;
    let client = reqwest::Client::new();

    let res = client.post(&format!("{}/repos", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": name,
            "description": desc,
            "visibility": "public"
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Create repository failed: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    // Extract repository ID
    let repo_id = json_resp["data"]["_id"].as_str()
        .or_else(|| json_resp["data"]["id"].as_str())
        .ok_or("Repository ID not found in server response.")?
        .to_string();

    // Save repository metadata locally in .gh-repo.json
    let meta = serde_json::json!({
        "name": name,
        "id": repo_id
    });
    std::fs::write(".gh-repo.json", serde_json::to_string_pretty(&meta).unwrap())
        .map_err(|e| format!("Failed to save repo config locally: {}", e))?;

    Ok(repo_id)
}

async fn handle_remote_push(config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let api_url = resolve_api_url(config).await;

    // Read target repository ID from a local file `.gh-repo.json`
    let repo_meta_path = ".gh-repo.json";
    if !std::path::Path::new(repo_meta_path).exists() {
        return Err("No remote repository configured. Run 'remote-create <repo_name>' in this folder first.".to_string());
    }
    
    let meta_content = std::fs::read_to_string(repo_meta_path)
        .map_err(|e| format!("Failed to read repo metadata: {}", e))?;
    let meta: serde_json::Value = serde_json::from_str(&meta_content)
        .map_err(|e| format!("Failed to parse repo metadata: {}", e))?;
    
    let repo_id = meta["id"].as_str().ok_or("Invalid repository ID in metadata file.")?;

    // Read gitignore patterns
    let mut gitignore_patterns = Vec::new();
    let gitignore_path = std::path::Path::new(&config.project_path).join(".gitignore");
    if gitignore_path.exists() {
        if let Ok(content) = std::fs::read_to_string(gitignore_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    gitignore_patterns.push(trimmed.to_string());
                }
            }
        }
    }

    let mut files = Vec::new();
    let mut it = WalkDir::new(&config.project_path).into_iter();

    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => return Err(format!("Error scanning directory: {}", err)),
            Some(Ok(entry)) => entry,
        };

        let path = entry.path();
        
        let relative_path = path.strip_prefix(&config.project_path)
            .map_err(|e| format!("Path error: {}", e))?
            .to_string_lossy()
            .to_string();

        if relative_path.is_empty() {
            continue;
        }

        // Apply gitignore matching
        if should_ignore(&relative_path, &gitignore_patterns) {
            if entry.file_type().is_dir() {
                it.skip_current_dir();
            }
            continue;
        }

        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let parent_path = path.parent()
            .unwrap_or(std::path::Path::new(""))
            .strip_prefix(&config.project_path)
            .unwrap_or(std::path::Path::new(""))
            .to_string_lossy()
            .to_string();

        if path.is_file() {
            // Read file content
            if let Ok(content) = std::fs::read_to_string(path) {
                files.push(serde_json::json!({
                    "name": name,
                    "path": relative_path,
                    "type": "file",
                    "content": content,
                    "parentPath": parent_path
                }));
            }
        } else if path.is_dir() {
            files.push(serde_json::json!({
                "name": name,
                "path": relative_path,
                "type": "dir",
                "content": "",
                "parentPath": parent_path
            }));
        }
    }

    println!("Uploading {} files/folders...", files.len());

    let client = reqwest::Client::new();
    let res = client.post(&format!("{}/repos/{}/sync", api_url, repo_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({ "files": files }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }

    Ok(())
}

async fn handle_remote_link(id: &str, config: &Config) -> Result<String, String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let api_url = resolve_api_url(config).await;
    let client = reqwest::Client::new();

    let res = client.get(&format!("{}/repos/{}", api_url, id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Get repository failed: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid response: {}", e))?;

    let name = json_resp["data"]["name"].as_str()
        .ok_or("Repository name not found in server response.")?
        .to_string();

    // Save repository metadata locally in .gh-repo.json
    let meta = serde_json::json!({
        "name": name,
        "id": id
    });
    std::fs::write(".gh-repo.json", serde_json::to_string_pretty(&meta).unwrap())
        .map_err(|e| format!("Failed to save repo config locally: {}", e))?;

    Ok(name)
}

async fn handle_remote_pull(config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let api_url = resolve_api_url(config).await;

    // Read target repository ID from local file `.gh-repo.json`
    let repo_meta_path = ".gh-repo.json";
    if !std::path::Path::new(repo_meta_path).exists() {
        return Err("No remote repository configured. Run 'remote-link <repo_id>' or 'remote-create <repo_name>' first.".to_string());
    }

    let meta_content = std::fs::read_to_string(repo_meta_path)
        .map_err(|e| format!("Failed to read repo metadata: {}", e))?;
    let meta: serde_json::Value = serde_json::from_str(&meta_content)
        .map_err(|e| format!("Failed to parse repo metadata: {}", e))?;

    let repo_id = meta["id"].as_str().ok_or("Invalid repository ID in metadata file.")?;

    println!("Pulling files for repository ID: {}...", repo_id);

    let client = reqwest::Client::new();
    let res = client.get(&format!("{}/repos/{}/contents", api_url, repo_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid JSON response: {}", e))?;

    let tree_data = json_resp["data"].as_array().ok_or("Invalid data format from server.")?;

    let base_path = std::path::Path::new(&config.project_path);

    for node in tree_data {
        write_tree_node(node, base_path)?;
    }

    Ok(())
}

fn write_tree_node(node: &serde_json::Value, base_path: &std::path::Path) -> Result<(), String> {
    let path_str = node["path"].as_str().ok_or("Node path missing")?;
    let node_type = node["type"].as_str().ok_or("Node type missing")?;
    let local_path = base_path.join(path_str);

    if node_type == "dir" {
        std::fs::create_dir_all(&local_path)
            .map_err(|e| format!("Failed to create directory {}: {}", local_path.display(), e))?;

        if let Some(children) = node["children"].as_array() {
            for child in children {
                write_tree_node(child, base_path)?;
            }
        }
    } else if node_type == "file" {
        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
        }
        let content = node["content"].as_str().unwrap_or("");
        std::fs::write(&local_path, content)
            .map_err(|e| format!("Failed to write file {}: {}", local_path.display(), e))?;
        println!("  {} {}", "->".green(), path_str);
    }
    Ok(())
}

fn should_ignore(relative_path: &str, patterns: &[String]) -> bool {
    let normalized = relative_path.replace('\\', "/");
    let parts: Vec<&str> = normalized.split('/').collect();
    
    if parts.contains(&".git") || parts.contains(&"config.toml") || parts.contains(&".gh-repo.json") {
        return true;
    }
    
    for pattern in patterns {
        let pattern_norm = pattern.replace('\\', "/");
        let pattern_clean = pattern_norm.trim_end_matches('/');
        
        if pattern_clean.starts_with("*.") {
            let ext = &pattern_clean[2..];
            if normalized.ends_with(ext) {
                return true;
            }
        }
        
        for part in &parts {
            if *part == pattern_clean {
                return true;
            }
        }
        
        if normalized.starts_with(pattern_clean) 
            || normalized.contains(&format!("/{}/", pattern_clean)) 
            || normalized.ends_with(&format!("/{}", pattern_clean)) 
        {
            return true;
        }
    }
    
    false
}

async fn resolve_api_url(config: &Config) -> String {
    let configured_url = config.api_url.clone().unwrap_or_else(|| "http://localhost:5000/api".to_string());
    if !configured_url.contains("localhost") && !configured_url.contains("127.0.0.1") {
        return configured_url;
    }
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(400))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
        
    let base_url = configured_url.trim_end_matches('/');
    let health_url = if base_url.ends_with("/api") {
        format!("{}/health", &base_url[..base_url.len()-4])
    } else {
        format!("{}/health", base_url)
    };
    
    match client.get(&health_url).send().await {
        Ok(resp) if resp.status().is_success() => configured_url,
        _ => "https://gtihub-backend.vercel.app/api".to_string()
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct SecretListItem {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    created_at: String,
}

async fn get_linked_repo_id() -> Result<String, String> {
    let repo_meta_path = ".gh-repo.json";
    if !std::path::Path::new(repo_meta_path).exists() {
        return Err("No remote repository configured. Run 'remote-link <repo_id>' or 'remote-create <repo_name>' first.".to_string());
    }
    
    let meta_content = std::fs::read_to_string(repo_meta_path)
        .map_err(|e| format!("Failed to read repo metadata: {}", e))?;
    let meta: serde_json::Value = serde_json::from_str(&meta_content)
        .map_err(|e| format!("Failed to parse repo metadata: {}", e))?;
    
    let repo_id = meta["id"].as_str().ok_or("Invalid repository ID in metadata file.")?.to_string();
    Ok(repo_id)
}

async fn handle_secret_set(name: &str, value: &str, config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let repo_id = get_linked_repo_id().await?;
    let api_url = resolve_api_url(config).await;
    
    let client = reqwest::Client::new();
    let res = client.post(&format!("{}/repos/{}/secrets", api_url, repo_id))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": name.to_uppercase(),
            "value": value
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }
    Ok(())
}

async fn handle_secret_list(config: &Config) -> Result<Vec<SecretListItem>, String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let repo_id = get_linked_repo_id().await?;
    let api_url = resolve_api_url(config).await;
    
    let client = reqwest::Client::new();
    let res = client.get(&format!("{}/repos/{}/secrets", api_url, repo_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid JSON response: {}", e))?;

    let secrets_arr = json_resp["data"].as_array().ok_or("Invalid secrets list format from server.")?;
    
    let list: Vec<SecretListItem> = serde_json::from_value(serde_json::Value::Array(secrets_arr.clone()))
        .map_err(|e| format!("Failed to parse secrets: {}", e))?;
        
    Ok(list)
}

async fn handle_secret_delete(name: &str, config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let repo_id = get_linked_repo_id().await?;
    let api_url = resolve_api_url(config).await;
    
    // First, list current secrets to find the ID corresponding to the name
    let secrets = handle_secret_list(config).await?;
    let target_name = name.to_uppercase();
    
    let target_secret = secrets.iter().find(|s| s.name == target_name)
        .ok_or_else(|| format!("Secret '{}' not found in this repository.", target_name))?;
        
    let client = reqwest::Client::new();
    let res = client.delete(&format!("{}/repos/{}/secrets/{}", api_url, repo_id, target_secret.id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }
    Ok(())
}

async fn handle_remote_pull_wiki(config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let repo_id = get_linked_repo_id().await?;
    let api_url = resolve_api_url(config).await;
    
    println!("Pulling wiki pages for repository ID: {}...", repo_id);
    let client = reqwest::Client::new();
    
    // 1. Fetch wiki page list
    let res = client.get(&format!("{}/repos/{}/wiki", api_url, repo_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned error: {}", err_text));
    }

    let json_resp: serde_json::Value = res.json()
        .await
        .map_err(|e| format!("Invalid JSON response: {}", e))?;

    let pages = json_resp["data"].as_array().ok_or("Invalid wiki pages format from server.")?;

    // Create target wiki directory locally
    let wiki_dir = std::path::Path::new(&config.project_path).join("wiki");
    std::fs::create_dir_all(&wiki_dir)
        .map_err(|e| format!("Failed to create local wiki directory: {}", e))?;

    for page in pages {
        let slug = page["slug"].as_str().ok_or("Missing page slug")?;
        
        // 2. Fetch page content detail
        let page_res = client.get(&format!("{}/repos/{}/wiki/{}", api_url, repo_id, slug))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if page_res.status().is_success() {
            let page_json: serde_json::Value = page_res.json().await.unwrap_or_default();
            if let Some(content) = page_json["data"]["content"].as_str() {
                let local_path = wiki_dir.join(format!("{}.md", slug));
                std::fs::write(&local_path, content)
                    .map_err(|e| format!("Failed to write local wiki page {}: {}", slug, e))?;
                println!("  {} wiki/{}.md", "->".green(), slug);
            }
        }
    }
    Ok(())
}

async fn handle_remote_push_wiki(config: &Config) -> Result<(), String> {
    let token = config.token.as_ref().ok_or("Not logged in. Please run 'login <email> <password>' first.")?;
    let repo_id = get_linked_repo_id().await?;
    let api_url = resolve_api_url(config).await;
    
    let wiki_dir = std::path::Path::new(&config.project_path).join("wiki");
    if !wiki_dir.exists() {
        return Err("No local 'wiki' directory found. Run 'remote-pull-wiki' first or create a 'wiki' folder with markdown files.".to_string());
    }

    println!("Scanning 'wiki' folder for pages to push...");
    let entries = std::fs::read_dir(&wiki_dir)
        .map_err(|e| format!("Failed to read wiki directory: {}", e))?;

    let client = reqwest::Client::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read wiki file {}: {}", filename, e))?;

            // Convert slug/filename to title format
            // e.g. "getting-started" -> "Getting Started"
            let title = filename
                .replace('-', " ")
                .replace('_', " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");

            println!("Pushing wiki page '{}'...", title);

            let res = client.post(&format!("{}/repos/{}/wiki", api_url, repo_id))
                .header("Authorization", format!("Bearer {}", token))
                .json(&serde_json::json!({
                    "title": title,
                    "content": content
                }))
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;

            if !res.status().is_success() {
                let err_text = res.text().await.unwrap_or_default();
                println!("  {} Failed to push '{}': {}", "x".red(), title, err_text);
            } else {
                println!("  {} Sync complete for '{}'", "✓".green(), title);
            }
        }
    }
    Ok(())
}






