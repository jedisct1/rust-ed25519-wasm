#ifndef WASM_NS_SIGN_H
#define WASM_NS_SIGN_H

#define CRYPTO_NAMESPACE(name) wasm_sign_##name
#define CRYPTO_SHARED_NAMESPACE(name) wasm_sign_shared_##name
#define _CRYPTO_NAMESPACE(name) _##wasm_sign_##name
#define _CRYPTO_SHARED_NAMESPACE(name) _##wasm_sign_shared_##name
#define CRYPTO_ALIGN(n) __attribute__((aligned(n)))

#endif
