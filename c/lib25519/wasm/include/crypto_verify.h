#ifndef WASM_CRYPTO_VERIFY_H
#define WASM_CRYPTO_VERIFY_H

#define crypto_verify crypto_verify_32
#define crypto_verify_BYTES 32

int crypto_verify(const unsigned char *,const unsigned char *);

#endif
