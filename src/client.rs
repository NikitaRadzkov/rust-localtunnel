use anyhow::Result;
use clap::Parser;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(name = "rust-localtunnel-client")]
#[command(about = "Rustunnel client - create tunnels to expose local services")]
struct ClientCli {
    #[arg(long, default_value = "http://localhost:8000")]
    server: String,

    #[arg(long, default_value = "http://localhost:8080")]
    target: String,

    #[arg(long)]
    subdomain: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = ClientCli::parse();

    println!("Connecting to Rustunnel server at {}...", cli.server);
    println!("Exposing local service: {}", cli.target);

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
        println!("\n✅ Tunnel created successfully!");
        println!("🌐 Public URL: {}", tunnel_info["public_url"]);
        println!("🎯 Target URL: {}", tunnel_info["target_url"]);
        println!("🆔 Tunnel ID: {}", tunnel_info["id"]);
        println!("\nPress Ctrl+C to stop the tunnel");

        // Keep the tunnel alive
        loop {
            sleep(Duration::from_secs(60)).await;
        }
    } else {
        let error_text = response.text().await?;
        eprintln!("❌ Failed to create tunnel: {}", error_text);
    }

    Ok(())
}
