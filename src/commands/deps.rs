use std::fs;
use std::path::Path;
use colored::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone)]
pub struct DepsData {
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}

pub fn get_data(project_path: &str) -> Option<DepsData> {
    let package_json_path = Path::new(project_path).join("package.json");
    
    if let Ok(content) = fs::read_to_string(package_json_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let mut deps = HashMap::new();
            if let Some(d) = json.get("dependencies").and_then(|d| d.as_object()) {
                for (k, v) in d {
                    deps.insert(k.clone(), v.as_str().unwrap_or("").to_string());
                }
            }

            let mut dev_deps = HashMap::new();
            if let Some(d) = json.get("devDependencies").and_then(|d| d.as_object()) {
                for (k, v) in d {
                    dev_deps.insert(k.clone(), v.as_str().unwrap_or("").to_string());
                }
            }

            return Some(DepsData {
                dependencies: deps,
                dev_dependencies: dev_deps,
            });
        }
    }
    None
}

pub fn run(project_path: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("{} parsing package.json in {}...\n", "→".blue().bold(), project_path.yellow()));

    if let Some(data) = get_data(project_path) {
        output.push_str(&format!("\n{}\n", "=== Project Dependencies ===".green().bold()));
        
        output.push_str(&format!("\n{}", "--- Dependencies ---".cyan().bold()));
        if data.dependencies.is_empty() {
            output.push_str("\nNone\n");
        } else {
            for (name, version) in &data.dependencies {
                output.push_str(&format!("\n{:<30} {}", name, version.yellow()));
            }
            output.push_str("\n");
        }

        output.push_str(&format!("\n{}", "--- Dev Dependencies ---".cyan().bold()));
        if data.dev_dependencies.is_empty() {
            output.push_str("\nNone\n");
        } else {
            for (name, version) in &data.dev_dependencies {
                output.push_str(&format!("\n{:<30} {}", name, version.yellow()));
            }
            output.push_str("\n");
        }
    } else {
        output.push_str(&format!("\n{}", "Error: Could not find or parse package.json".red().bold()));
    }

    output
}
