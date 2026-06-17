#ifndef WASM_CRYPTO_POW_H
#define WASM_CRYPTO_POW_H

#define crypto_pow crypto_pow_inv25519
#define crypto_pow_BYTES 32

void crypto_pow(unsigned char *,const unsigned char *);

#endif
