# ed25519-wasm

High-performance Ed25519 signatures for WebAssembly, backed by `lib25519`.

The API mirrors the common one-shot parts of `ed25519-compact`:

```rust
use ed25519_wasm::{KeyPair, Noise, Seed};

let key_pair = KeyPair::from_seed(Seed::generate());
let message = b"hello";
let signature = key_pair.sk.sign(message, None);

key_pair.pk.verify(message, &signature).unwrap();
```

## Performance

Benchmarks were run under Wasmtime 45.0.2, using 128-byte messages.

The table reports median time per operation and relative speed compared to this crate.

| Algorithm / implementation             | Sign 128B | Verify 128B | Sign vs this crate | Verify vs this crate |
| -------------------------------------- | --------: | ----------: | -----------------: | -------------------: |
| Ed25519 / this crate                   |   15.1 us |     49.1 us |               1.0x |                 1.0x |
| Ed25519 / libsodium wasm               |   21.4 us |     53.1 us |        1.4x slower |          1.1x slower |
| ECDSA-P256/SHA-256 / RustCrypto `p256` |    245 us |      277 us |       16.2x slower |          5.6x slower |
| ML-DSA-44 / Zig std wasm               |  135.5 us |     23.3 us |        8.9x slower |          2.1x faster |
| ML-DSA-44 / RustCrypto `ml-dsa`        |    706 us |     91.7 us |       46.6x slower |          1.9x slower |
| RSA-2048 / RustCrypto `rsa`            |   3.21 ms |      171 us |        212x slower |          3.5x slower |
| RSA-3072 / RustCrypto `rsa`            |   10.2 ms |      394 us |        671x slower |          8.0x slower |

## Rebuilding the library

`build.rs` links `wasm-libs/libed25519.a` for `wasm32` targets

The archive is reproducible with:

```sh
cd wasm-libs
zig build -Drelease
cp zig-out/lib/libed25519.a .
```

The bundled C source subset under `c/lib25519` is local to this crate.
