#ifndef WASM_CRYPTO_MGNP_ED25519_H
#define WASM_CRYPTO_MGNP_ED25519_H

#define crypto_mGnP_ed25519_MBYTES 32
#define crypto_mGnP_ed25519_NBYTES 64
#define crypto_mGnP_ed25519_PBYTES 32
#define crypto_mGnP_ed25519_OUTPUTBYTES 33

void crypto_mGnP_ed25519(unsigned char *,const unsigned char *,const unsigned char *,const unsigned char *);

#endif
