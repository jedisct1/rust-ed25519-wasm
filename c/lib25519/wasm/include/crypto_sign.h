#ifndef WASM_CRYPTO_SIGN_H
#define WASM_CRYPTO_SIGN_H

#define crypto_sign_SECRETKEYBYTES 64
#define crypto_sign_PUBLICKEYBYTES 32
#define crypto_sign_BYTES 64

void crypto_sign_keypair(unsigned char *,unsigned char *);
void crypto_sign(unsigned char *,long long *,const unsigned char *,long long,const unsigned char *);
int crypto_sign_open(unsigned char *,long long *,const unsigned char *,long long,const unsigned char *);

#endif
