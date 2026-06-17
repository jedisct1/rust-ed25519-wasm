#ifndef WASM_NS_POW_H
#define WASM_NS_POW_H

#define CRYPTO_NAMESPACE(name) wasm_pow_##name
#define CRYPTO_SHARED_NAMESPACE(name) wasm_pow_shared_##name
#define _CRYPTO_NAMESPACE(name) _##wasm_pow_##name
#define _CRYPTO_SHARED_NAMESPACE(name) _##wasm_pow_shared_##name
#define CRYPTO_ALIGN(n) __attribute__((aligned(n)))

#endif
