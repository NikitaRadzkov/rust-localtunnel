# rust-localtunnel

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A modern, high-performance localtunnel implementation written in Rust. Expose your local development server to the internet with a public URL.

## Features

- 🚀 **Fast and Efficient**: Built with Rust for maximum performance
- 🔒 **Secure**: Proper handling of HTTP headers and proxy requests
- 🎯 **Simple**: Easy-to-use CLI tools for both server and client
- 🔧 **Customizable**: Support for custom subdomains
- ⚡ **Async**: Built on Tokio for excellent concurrency

## Installation

### From Source

```bash
git clone https://github.com/yourusername/rust-localtunnel.git
cd rust-localtunnel
cargo build --release
```

The compiled binaries will be in `target/release/`:
- `rust-localtunnel` - The tunnel server
- `rust-localtunnel-client` - The client tool

## Usage

### Running the Server

Start the tunnel server:

```bash
./target/release/rust-localtunnel --port 8000 --host localhost
```

Options:
- `--port`: Server port (default: 8000)
- `--host`: Server host (default: localhost)
- `--target-port`: Default target port (default: 8080)
- `--subdomain`: Optional subdomain hint

### Creating a Tunnel

Use the client to create a tunnel to your local service:

```bash
./target/release/rust-localtunnel-client \
  --server http://localhost:8000 \
  --target http://localhost:8080 \
  --subdomain myapp
```

Options:
- `--server`: Tunnel server URL (default: http://localhost:8000)
- `--target`: Local service URL to expose (default: http://localhost:8080)
- `--subdomain`: Optional subdomain name (random if not provided)

### Example Workflow

1. Start a local development server:
   ```bash
   python -m http.server 8080
   ```

2. Start the tunnel server:
   ```bash
   ./target/release/rust-localtunnel --port 8000
   ```

3. Create a tunnel:
   ```bash
   ./target/release/rust-localtunnel-client \
     --target http://localhost:8080 \
     --subdomain myapp
   ```

4. Access your local server via the public URL shown in the output.

## API

### Create Tunnel

```http
POST /api/tunnels
Content-Type: application/json

{
  "target_url": "http://localhost:8080",
  "subdomain": "myapp"  // optional
}
```

Response:
```json
{
  "id": "tunnel-id",
  "subdomain": "myapp",
  "public_url": "https://myapp.rustunnel.example.com",
  "target_url": "http://localhost:8080"
}
```

## Architecture

The project consists of two main components:

1. **Server** (`rust-localtunnel`): Handles tunnel creation and proxies HTTP requests from public URLs to local services.
2. **Client** (`rust-localtunnel-client`): Connects to the server and creates tunnels to expose local services.

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Development Mode

```bash
# Server
RUST_LOG=info cargo run --bin rust-localtunnel -- --port 8000

# Client
cargo run --bin rust-localtunnel-client -- --target http://localhost:8080
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [localtunnel](https://github.com/localtunnel/localtunnel)
- Built with [Tokio](https://tokio.rs/), [Warp](https://github.com/seanmonstar/warp), and other amazing Rust libraries

## Support

If you encounter any issues or have questions, please open an issue on [GitHub](https://github.com/yourusername/rust-localtunnel/issues).

