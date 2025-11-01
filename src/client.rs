use anyhow::Result;
use clap::Parser;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser)]
#[command(name - "rust-localtunnel")]
struct ClientCli {
    #[arg(short, long, default_value = "http://localhost:8000")]
    server: String,

    #[arg(short, long, default_value = "http://localhost:8080")]
    target: String,

    #[arg(short, long)]
    subdomain: Option<String>,
}

pub async fn run_client() -> Result<()> {
    let cli = ClientCli::parse();

    println!("Connecting to Rustunnel server...");

    let client = reqwest::Client::new();
    let request = json!({
        "target_url": cli.target,
        "subdomain": cli.subdomain,
    });

    let response = client
        .post(&format!("{}/api/tunnels", cli.server))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let tunnel_info: serde_json::Value = response.json().await?;
        println!("Tunnel created successfully!");
        println!("Public URL: {}", tunnel_info["public_url"]);
        println!("Target URL: {}", tunnel_info["target_url"]);
        println!("Tunnel ID: {}", tunnel_info["id"]);
        println!("\nPress Ctrl+C to stop the tunnel");

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    } else {
        eprintln!("Failed to create tunnel: {}", response.status());
    }

    Ok(())
}
