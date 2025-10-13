# transport-api-wrapper

A Rust wrapper for the `transport.rest` API, providing a simple and easy-to-use interface for interacting with public transport data.

## Library Usage

To use this crate as a library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
transport-api-wrapper = "0.1.0"
```

Then, you can use the `Client` to interact with the API:

```rust
use transport_api_wrapper::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let locations = client.locations("berlin").await.unwrap();
    println!("{:#?}", locations);
}
```

## CLI Usage

This crate also provides a command-line interface (CLI) for interacting with the API. To install it, you will need to enable the `cli` feature:

```bash
cargo install transport-api-wrapper --features cli
```

Once installed, you can use the `transporter` command to find locations and get departures:

```bash
transporter locations "berlin"
transporter departures "8011160"
```