[![Rust](https://img.shields.io/badge/rust-1.92-orange.svg)](https://www.rust-lang.org/)
# Crabby Buddy
A simple command-line interface for chatting with local LLMs, built in Rust. It runs GGUF models locally via [llama.cpp](https://github.com/ggerganov/llama.cpp) (through the `llama-cpp-2` bindings) — no API, no network.

## Features

- Real-time AI chat through the terminal
- Runs local GGUF models on CPU/GPU (GPU offload enabled by default)
- Persistent sessions with fast KV-cache resume, plus an incognito mode
- Interactive arrow-key picker for models and sessions
- Vi-style line editing and input history

## Installation

### From source
```bash
git clone https://github.com/AshLink95/CrabbyBuddy.git
cd CrabbyBuddy
cargo build --release
```
For an optimized build on Linux, if you're using an nvidia gpu, consider:
```bash
RUSTFLAGS="-C target-cpu=native -C lto=fat" cargo build --release --target x86_64-unknown-linux-gnu --features cuda
```
> [!note] Not using a nvidia gpu
> Drop the `--features` flag

With these flags, the binary should be at:
```
CrabbyBuddy/target/x86_64-unknown-linux-gnu/release/crabbybuddy
```

## Setup
Crabby Buddy reads models and stores sessions under your local data directory:
```
$XDG_DATA_HOME/crabbybuddy/     (typically ~/.local/share/crabbybuddy/)
├── models/     # put your .gguf model files here
└── sessions/   # persistent sessions are saved here automatically
```
Drop one or more `.gguf` files into `models/` before running.

## Usage
```bash
# Start chatting
crabbybuddy

# Show help
crabbybuddy --help
```
On launch you'll pick a model, then choose a session: **go incognito** (nothing saved), an existing saved session, or **new chat** (creates a persistent session). Navigate with the arrow keys (or `j`/`k`) and press Enter to select.

Type your messages and press Enter. Type `\bye!` or press `Ctrl-C` to exit.
