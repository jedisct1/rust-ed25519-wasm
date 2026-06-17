#ifndef WASM_CRYPTO_MGNP_H
#define WASM_CRYPTO_MGNP_H

#define crypto_mGnP crypto_mGnP_ed25519
#define crypto_mGnP_MBYTES 32
#define crypto_mGnP_NBYTES 64
#define crypto_mGnP_PBYTES 32
#define crypto_mGnP_OUTPUTBYTES 33

void crypto_mGnP(unsigned char *,const unsigned char *,const unsigned char *,const unsigned char *);

#endif
