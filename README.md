# DigitalOcean Rust Client

Auto-generated Rust client for the [DigitalOcean API](https://docs.digitalocean.com/reference/api/api-reference/).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
digitalocean = { git = "https://github.com/iwatakeshi/do-rs" }
```

Or with specific features:

```toml
[dependencies]
# Use rustls instead of native-tls
digitalocean = { git = "https://github.com/iwatakeshi/do-rs", default-features = false, features = ["rustls-tls"] }
```

## Usage

```rust
use digitalocean::apis::configuration::Configuration;
use digitalocean::apis::account_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration with your API token
    let mut config = Configuration::new();
    config.bearer_access_token = Some(std::env::var("DIGITALOCEAN_TOKEN")?);

    // Get account information
    let account = account_api::account_get(&config).await?;
    println!("Account: {:?}", account);

    Ok(())
}
```

## Features

- **native-tls** (default): Use the platform's native TLS implementation
- **rustls-tls**: Use rustls for TLS

## Documentation

To build and view the Rust documentation locally:

```bash
# Build the documentation
cargo doc --no-deps

# Open the documentation in your browser
cargo doc --no-deps --open
```

The documentation includes all API endpoints, models, and configuration options with detailed descriptions.

## API Coverage

This client provides access to all DigitalOcean API endpoints including:

- Account management
- Droplets (virtual machines)
- Kubernetes clusters
- Databases
- Spaces (object storage)
- Domains and DNS
- Load Balancers
- VPCs
- Firewalls
- And more...

For complete API documentation, see the [DigitalOcean API Reference](https://docs.digitalocean.com/reference/api/api-reference/).

## Code Generation

This client is auto-generated from the official [DigitalOcean OpenAPI specification](https://github.com/digitalocean/openapi) using [OpenAPI Generator](https://openapi-generator.tech/).

To regenerate the client:

1. Clone this repository
2. Run the generation script (requires Node.js and Java):
   ```bash
   npm install @openapitools/openapi-generator-cli
   npx @openapitools/openapi-generator-cli generate \
     -i https://api-engineering.nyc3.cdn.digitaloceanspaces.com/spec-ci/DigitalOcean-public.v2.yaml \
     -g rust \
     -o . \
     --additional-properties=packageName=digitalocean,packageVersion=0.1.0
   ```

## License

MIT License - See [LICENSE](LICENSE) for details.
