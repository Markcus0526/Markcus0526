// src/main.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions};
use std::io::{Read, Write, Seek}; 
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
    owner: Owner,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    login: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    items: Vec<Repository>,
}

fn format_stars(count: u32) -> String {
    if count >= 1000 {
        format!("{:.1}k", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}

async fn fetch_trending_rust_repos() -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.github.com/search/repositories?q=language:rust&sort=stars&order=desc&per_page=10";
    
    let response: Response = client
        .get(url)
        .header("User-Agent", "rust-trending-repos")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response.items)
}

async fn fetch_trending_rust_agent_repos() -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.github.com/search/repositories?q=language:rust+agent+OR+ai+OR+autonomous+in:name,description&sort=stars&order=desc&per_page=10";
    
    let response: Response = client
        .get(url)
        .header("User-Agent", "rust-trending-repos")
        .send()
        .await?
        .json()
        .await?;
    
    Ok(response.items)
}

fn append_to_readme(trending_repos: Vec<Repository>, agent_repos: Vec<Repository>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("../README.md")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let header = "## Rust Trending";

    // If you want to clear everything AFTER the header to refresh the list:
    if let Some(start_index) = content.find(header) {
        let after_header = start_index;
        content.truncate(after_header); // Removes everything after the header
    }

    content.push_str(&format!("{} ({})\n\n", header, Local::now().format("%Y-%m-%d %H:%M:%S")));

    let git_prefix = "https://www.github.com/";

    // Append top Rust repositories
    content.push_str("### Top Trending Rust Repositories\n\n");
    content.push_str("| Name | Developer | Stars | Description |\n");
    content.push_str("|------|-----------|-------|-------------|\n");
    
    for repo in trending_repos {
        let description = repo.description.unwrap_or_default();
        content.push_str(&format!(
            "| [{}]({}) | {} | {} | {} |\n",
            repo.name,
            git_prefix.to_owned() + &repo.full_name,
            repo.owner.login,
            format_stars(repo.stargazers_count),
            description
        ));
    }
    
    // Append Rust agent repositories
    content.push_str("\n### Top Trending Rust Agent Repositories\n\n");
    content.push_str("| Name | Developer | Stars | Description |\n");
    content.push_str("|------|-----------|-------|-------------|\n");
    
    for repo in agent_repos {
        let description = repo.description.unwrap_or_default();
        content.push_str(&format!(
            "| [{}]({}) | {} | {} | {} |\n",
            repo.name,
            git_prefix.to_owned() + &repo.full_name,
            repo.owner.login,
            format_stars(repo.stargazers_count),
            description
        ));
    }
    
    // Truncate and write back to file
    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let trending_repos = fetch_trending_rust_repos().await?;
    let agent_repos = fetch_trending_rust_agent_repos().await?;
    
    append_to_readme(trending_repos, agent_repos)?;
    
    println!("README.md updated successfully!");
    Ok(())
}
