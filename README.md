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

<!-- ## On the GPU layer problem -->
<!---->
<!-- This is a real headache. Here's what you're facing: -->
<!---->
<!-- **The core issue:** Layer counts vary wildly: -->
<!-- - Llama 2 7B: 32 layers -->
<!-- - Llama 2 13B: 40 layers   -->
<!-- - Mistral 7B: 32 layers -->
<!-- - Mixtral 8x7B: 32 layers -->
<!-- - Different quants of the same model behave differently under VRAM pressure -->
<!---->
<!-- **Solutions:** -->
<!---->
<!-- ### 1. **Parse GGUF metadata** (most reliable) -->
<!-- GGUF files have metadata embedded. Libraries like `gguf-py` or `llm-sys` can read it without loading the whole model: -->
<!-- ```rust -->
<!-- // Pseudocode - use a GGUF parser to extract: -->
<!-- let metadata = parse_gguf_header(model_path); -->
<!-- let layer_count = metadata.get("llama.block_count") -->
<!--     .or_else(|| metadata.get("mistral.block_count")) -->
<!--     .unwrap_or_default(); -->
<!-- ``` -->
<!---->
<!-- ### 2. **Auto-detect based on VRAM** (practical approach) -->
<!-- Start conservative, measure actual VRAM usage, adapt: -->
<!-- ```rust -->
<!-- fn estimate_gpu_layers( -->
<!--     model_params: &ModelInfo, -->
<!--     available_vram_mb: u32, -->
<!-- ) -> u32 { -->
<!--     let bytes_per_layer = model_params.estimated_layer_size(); -->
<!--     let safe_threshold = (available_vram_mb as f32 * 0.85) as u32; // Leave 15% headroom -->
<!---->
<!--     let estimated_layers = safe_threshold / bytes_per_layer; -->
<!--     std::cmp::min(estimated_layers, model_params.total_layers) -->
<!-- } -->
<!-- ``` -->
<!---->
<!-- ### 3. **User config with smart defaults** -->
<!-- Let users override but provide auto-detection: -->
<!-- ```toml -->
<!-- [model] -->
<!-- name = "mistral-7b.gguf" -->
<!-- gpu_layers = "auto"  # or explicit: 32 -->
<!---->
<!-- [detection] -->
<!-- gpu_memory_percent = 0.85 -->
<!-- ``` -->
<!---->
<!-- ### 4. **Runtime feedback loop** -->
<!-- Let it fail gracefully and adapt: -->
<!-- ```rust -->
<!-- match load_with_layers(gpu_layers) { -->
<!--     Ok(_) => gpu_layers, -->
<!--     Err(OutOfMemory) => { -->
<!--         eprintln!("Reducing layers from {} to {}", gpu_layers, gpu_layers - 4); -->
<!--         load_with_layers(gpu_layers - 4)? -->
<!--     } -->
<!-- } -->
<!-- ``` -->
