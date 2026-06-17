#ifndef WASM_CRYPTO_NG_H
#define WASM_CRYPTO_NG_H

#define crypto_nG crypto_nG_merged25519
#define crypto_nG_SCALARBYTES 32
#define crypto_nG_POINTBYTES 32

void crypto_nG(unsigned char *,const unsigned char *);

#endif
