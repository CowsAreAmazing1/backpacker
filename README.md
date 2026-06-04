Build & run (wasm-web) instructions

Prerequisites:
- Rust toolchain with the `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-bindgen-cli` installed: `cargo install -f wasm-bindgen-cli`
- A static file server (e.g., Python `python -m http.server`, or `basic-http-server` via `cargo install basic-http-server`)

Build steps:

1. Build the wasm artifact:

```bash
cargo build --target wasm32-unknown-unknown --release
```

2. Generate JS/WASM bindings with `wasm-bindgen` (outputs into `pkg/`):

```bash
wasm-bindgen target/wasm32-unknown-unknown/release/backpacker.wasm --out-dir pkg --target web --out-name backpacker
```

3. Host the directory:

```bash
python -m http.server 8080
ngrok http 3030
```

In practice:

1. Run `build_server.bat` in one terminal.
2. Host the directory in another.