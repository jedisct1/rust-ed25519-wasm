#ifndef WASM_NS_MGNP_H
#define WASM_NS_MGNP_H

#define CRYPTO_NAMESPACE(name) wasm_mgnp_##name
#define CRYPTO_SHARED_NAMESPACE(name) wasm_mgnp_shared_##name
#define _CRYPTO_NAMESPACE(name) _##wasm_mgnp_##name
#define _CRYPTO_SHARED_NAMESPACE(name) _##wasm_mgnp_shared_##name
#define CRYPTO_ALIGN(n) __attribute__((aligned(n)))

#endif
