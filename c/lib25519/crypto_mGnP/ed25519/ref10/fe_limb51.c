#include <stdint.h>
#include <string.h>

#include "fe.h"

typedef uint64_t fe25519[5];
#define fe25519_frombytes CRYPTO_NAMESPACE(limb51_frombytes)
#define fe25519_tobytes CRYPTO_NAMESPACE(limb51_tobytes)
void fe25519_frombytes(fe25519, const unsigned char *);
void fe25519_tobytes(unsigned char *, const fe25519);

#include "limb51/sodium_fe51.h"
#include "limb51/sodium_fe51_bytes.h"

void fe_frombytes(fe h, const unsigned char *s) { fe25519_frombytes(h, s); }
void fe_tobytes(unsigned char *s, const fe h) { fe25519_tobytes(s, h); }
void fe_0(fe h) { fe25519_0(h); }
void fe_1(fe h) { fe25519_1(h); }
void fe_copy(fe h, const fe f) { fe25519_copy(h, f); }
void fe_cmov(fe h, const fe f, unsigned int b) { fe25519_cmov(h, f, b); }
void fe_cswap(fe h, fe f, unsigned int b) { fe25519_cswap(h, f, b); }
void fe_add(fe h, const fe f, const fe g) { fe25519_add(h, f, g); }
void fe_sub(fe h, const fe f, const fe g) { fe25519_sub(h, f, g); }
void fe_neg(fe h, const fe f) { fe25519_neg(h, f); }
void fe_mul(fe h, const fe f, const fe g) { fe25519_mul(h, f, g); }
void fe_sq(fe h, const fe f) { fe25519_sq(h, f); }
void fe_sq2(fe h, const fe f) { fe25519_sq2(h, f); }
void fe_mul121666(fe h, const fe f) { fe25519_mul32(h, f, 121666); }
int fe_isnegative(const fe f) { return fe25519_isnegative(f); }
int fe_isnonzero(const fe f) { return 1 - fe25519_iszero(f); }
