### helpful commands:

#### build docs

```bash
cargo doc --bin demo
cargo doc --bin ipld --target wasm32-wasi
```

#### build example wasm

```bash
# builds the rust to wasm
cargo wasi build --bin sampleipld
# builds the wasm to native .so
lucetc \
  target/wasm32-wasi/debug/sampleipld.wasm \
  --output target/wasm32-wasi/debug/sampleipld.so \
  --bindings ~/gitdev/datalove-vendor/lucet/lucet-wasi/bindings.json \
  --reserved-size 64MiB \
  --opt-level 0 \
  --wasi_exe
```

#### build demo runtime

```
env \
  APP_NAME=sampleipld \
  APP_LOCATION=target/wasm32-wasi/debug/sampleipld.so \
  RUST_BACKTRACE=full \
  RUST_LOG=trace \
  RUSTFLAGS="-C link-args=-Wl,--export-dynamic" \
  cargo run --bin demo
```
