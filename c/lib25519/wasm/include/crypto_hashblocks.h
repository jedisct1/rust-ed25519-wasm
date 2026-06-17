#ifndef WASM_CRYPTO_HASHBLOCKS_H
#define WASM_CRYPTO_HASHBLOCKS_H

#define crypto_hashblocks crypto_hashblocks_sha512
#define crypto_hashblocks_STATEBYTES 64
#define crypto_hashblocks_BLOCKBYTES 128

int crypto_hashblocks(unsigned char *,const unsigned char *,long long);

#endif
