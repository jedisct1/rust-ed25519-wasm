#include <string.h>

#include "../crypto_nG/merged25519/ref10/ge.h"
#include "crypto_nG_merged25519.h"
#include "crypto_uint8.h"

void crypto_nG_merged25519(unsigned char *pk,const unsigned char *sk)
{
  unsigned char e[32];
  ge_p3 A;
  fe recip;
  fe x;
  fe y;

  memcpy(e,sk,32);
  e[31] &= 127;

  ge_scalarmult_base(&A,e);

  fe_invert(recip,A.Z);
  fe_mul(y,A.Y,recip);
  fe_tobytes(pk,y);

  fe_mul(x,A.X,recip);
  pk[31] ^= crypto_uint8_shlmod(fe_isnegative(x),7);
}
