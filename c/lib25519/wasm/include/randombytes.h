#ifndef WASM_RANDOMBYTES_H
#define WASM_RANDOMBYTES_H

void randombytes(unsigned char *,long long);
void randombytes_seed(unsigned long long);
void randombytes_seed_bytes(const unsigned char *,long long);

#endif
