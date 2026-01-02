[![Rust](https://img.shields.io/badge/rust-1.92-orange.svg)](https://www.rust-lang.org/)
# Crabby Buddy

A simple command-line interface for interacting with AI chat services, built in Rust. It uses [ApiFreeLLM](https://apifreellm.com/) as its AI api.

## Features

- Real-time AI chat through terminal
- Automatic retry on connection failures (might have to `<C-c>` sometimes)
- Reusable API client library
- Async/await with Tokio

## Installation

### From source
```bash
git clone https://github.com/AshLink95/CrabbyBuddy.git
cd CrabbyBuddy
cargo build --release
```

For an optimized build on linux, consider
```bash
RUSTFLAGS="-C target-cpu=native -C lto=fat" cargo build --release --target x86_64-unknown-linux-gnu
```

Using the optimal build flag, the binary will be available at 
```
Crabbybuddy/target/x86_64-unknown-linux-gnu/release/crabbybuddy
```

## Usage

### As a CLI tool
```bash
# Start chatting
crabbybuddy

# Show help
crabbybuddy --help
```

Type your messages and press Enter. Type `Bye!` to exit.

### As a library
Add to your `Cargo.toml`:
```toml
[dependencies]
crabbybuddy = "0.1.0"
```

In your code, you can follow this example:
```rust
use crabbybuddy::ApiFreeLLM;

#[tokio::main]
async fn main() {
    let response = ApiFreeLLM::new("Hello, AI!").await;
    println!("{}", response.get_resp());
}
```
or, check `main.rs`!
