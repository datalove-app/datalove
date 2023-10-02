# Datalove

## Why

- user identity :
  - should be user-owned, not blockchain-owned
- apps:
  - should be self-hosted, offline-first and p2p (uncensorable, immortal) by default
  - should be simple to build, trivial to scale

## Components:

`persona`: distributed user identity
- each device initializes a device+persona specific keypair (if shared, unlocked on the device by biometrics/password)
  - examples:
    - macbooks can be shared --> no unlock (after login) required
    - phones are user-specific --> no unlock (after faceid/touchid/etc) required
    - server

<!-- ## Toolchain

guest-native toolchain(s):
  compilers:
    lean4 -> olean (can be eval'ed by lean4-wasix)
    (eventually) lean4/olean -> wasm component
    ? lean4 -> binaryen-ir

host-native toolchain(s):
  compilers:
    rustc_clang(c/c++/rust)
  outputs:
    wasm component(s)
    runtime binaries (native)
  - (initial) lean4-wasix
    outputs: custom interpreter/binary
  - runtime (wgpu, wasi-crypto, etc host fns)
    outputs:
      wit/wai
      wasm component interfaces

  ? wasm-opt (crate)
  ? wasm-binaryen

  - wasi-sdk, wasix-libc -->
