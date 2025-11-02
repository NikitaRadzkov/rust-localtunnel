use anyhow::Result;
use clap::Parser;
use log::{error, info};
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::Filter;
use bytes::Bytes;
use warp::http::header::HeaderMap;

type Subdomains = Arc<RwLock<HashMap<String, Tunnel>>>;

#[derive(Debug, Clone)]
struct Tunnel {
    target_url: String,
}

#[derive(Parser, Debug)]
#[command(name = "rustunnel")]
#[command(about = "Modern localtunnel implementation in Rust")]
struct Cli {
    #[arg(long, default_value = "8000")]
    port: u16,

    #[arg(long, default_value = "localhost")]
    host: String,

    #[arg(long, default_value = "8080")]
    target_port: u16,

    #[arg(long)]
    subdomain: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let subdomains: Subdomains = Arc::new(RwLock::new(HashMap::new()));

    info!("Starting Rustunnel server on {}:{}", cli.host, cli.port);

    // API routes for tunnel management
    let api_routes = warp::path("api")
        .and(warp::path("tunnels"))
        .and(warp::post())
        .and(json_body())
        .and(with_subdomains(subdomains.clone()))
        .and_then(create_tunnel_handler);

    // Proxy routes for tunneling traffic
    let proxy_routes = warp::any()
        .and(warp::host::optional())
        .and(warp::path::full())
        .and(warp::method())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and(with_subdomains(subdomains.clone()))
        .and_then(proxy_handler);

    let routes = api_routes.or(proxy_routes);

    let addr = (cli.host.as_str(), cli.port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("Invalid host/port");

    warp::serve(routes)
        .run(addr)
        .await;

    Ok(())
}

fn with_subdomains(
    subdomains: Subdomains,
) -> impl Filter<Extract = (Subdomains,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || subdomains.clone())
}

fn json_body() -> impl Filter<Extract = (CreateTunnelRequest,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(serde::Deserialize)]
struct CreateTunnelRequest {
    target_url: String,
    subdomain: Option<String>,
}

#[derive(serde::Serialize)]
struct CreateTunnelResponse {
    id: String,
    subdomain: String,
    public_url: String,
    target_url: String,
}

async fn create_tunnel_handler(
    req: CreateTunnelRequest,
    subdomains: Subdomains,
) -> Result<impl warp::Reply, warp::Rejection> {
    let subdomain = req.subdomain.unwrap_or_else(|| Uuid::new_v4().to_string());
    let tunnel_id = Uuid::new_v4().to_string();

    let tunnel = Tunnel {
        target_url: req.target_url.clone(),
    };

    {
        let mut tunnels = subdomains.write().await;
        if tunnels.contains_key(&subdomain) {
            return Err(warp::reject::custom(SubdomainTaken));
        }
        tunnels.insert(subdomain.clone(), tunnel);
    }

    let response = CreateTunnelResponse {
        id: tunnel_id,
        subdomain: subdomain.clone(),
        public_url: format!("https://{}.rustunnel.example.com", subdomain),
        target_url: req.target_url,
    };

    info!("Created tunnel: {} -> {}", response.public_url, response.target_url);

    Ok(warp::reply::json(&response))
}

// Helper function to extract authority from host
fn extract_authority(host: Option<String>) -> Option<String> {
    host.and_then(|h| {
        // Remove port if present
        h.split(':').next().map(|s| s.to_string())
    })
}

async fn proxy_handler(
    host: Option<warp::http::uri::Authority>,
    path: warp::path::FullPath,
    method: warp::http::Method,
    headers: HeaderMap,
    body: Bytes,
    subdomains: Subdomains,
) -> Result<impl warp::Reply, warp::Rejection> {
    let host_str = host
        .as_ref()
        .map(|h| h.as_str().to_string())
        .ok_or_else(|| warp::reject::not_found())?;

    let authority = extract_authority(Some(host_str.clone()))
        .ok_or_else(|| warp::reject::not_found())?;
    let subdomain = extract_subdomain(&authority).ok_or_else(|| warp::reject::not_found())?;

    let tunnel = {
        let tunnels = subdomains.read().await;
        tunnels.get(&subdomain).cloned()
    };

    let tunnel = tunnel.ok_or_else(|| warp::reject::not_found())?;

    info!("Proxying request to tunnel: {} {}", method, path.as_str());

    // Forward the request to the target URL
    let client = reqwest::Client::new();
    let target_url = format!("{}{}", tunnel.target_url, path.as_str());

    let mut request_builder = client.request(method, &target_url);

    // Copy headers (excluding hop-by-hop headers)
    for (key, value) in headers.iter() {
        if !is_hop_by_hop_header(key) {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder
        .body(body)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to proxy request: {}", e);
            warp::reject::not_found()
        })?;

    let status = response.status();
    let headers = response.headers().clone();
    let body_bytes = response.bytes().await.map_err(|_| warp::reject::not_found())?;

    // Create response with bytes
    let mut response_builder = warp::http::Response::builder()
        .status(status);

    // Copy response headers
    for (key, value) in headers.iter() {
        if !is_hop_by_hop_header(key) {
            response_builder = response_builder.header(key, value);
        }
    }

    let response = response_builder.body(body_bytes).map_err(|e| {
        error!("Failed to build response: {}", e);
        warp::reject::not_found()
    })?;

    Ok(response)
}

fn extract_subdomain(host: &str) -> Option<String> {
    host.split('.')
        .next()
        .filter(|&s| !s.is_empty() && s != "www" && s != "localhost")
        .map(|s| s.to_string())
}

fn is_hop_by_hop_header(header_name: &warp::http::HeaderName) -> bool {
    matches!(
        header_name.as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
    )
}

#[derive(Debug)]
struct SubdomainTaken;

impl warp::reject::Reject for SubdomainTaken {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_subdomain() {
        assert_eq!(
            extract_subdomain("myapp.rustunnel.example.com"),
            Some("myapp".to_string())
        );
        assert_eq!(
            extract_subdomain("rustunnel.example.com"),
            None
        );
        assert_eq!(extract_subdomain("localhost"), None);
        assert_eq!(extract_subdomain("www.example.com"), None);
    }

    #[test]
    fn test_extract_authority() {
        assert_eq!(
            extract_authority(Some("myapp.rustunnel.example.com:8080".to_string())),
            Some("myapp.rustunnel.example.com".to_string())
        );
        assert_eq!(
            extract_authority(Some("localhost:3000".to_string())),
            Some("localhost".to_string())
        );
    }
}
