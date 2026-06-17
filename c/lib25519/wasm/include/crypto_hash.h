#ifndef WASM_CRYPTO_HASH_H
#define WASM_CRYPTO_HASH_H

#define crypto_hash crypto_hash_sha512
#define crypto_hash_BYTES 64

void crypto_hash(unsigned char *,const unsigned char *,long long);

#endif
