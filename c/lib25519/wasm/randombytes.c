#include "randombytes.h"

#include <stddef.h>
#include <stdint.h>
#include <wasi/api.h>

static int randombytes_seeded = 0;

static unsigned long long randombytes_state[4] = {
  0x9e3779b97f4a7c15ULL,
  0xbf58476d1ce4e5b9ULL,
  0x94d049bb133111ebULL,
  0xd1b54a32d192ed03ULL
};

static unsigned long long rotl(const unsigned long long x,int k)
{
  return (x << k) | (x >> (64 - k));
}

static unsigned long long splitmix64(unsigned long long *x)
{
  unsigned long long z;

  *x += 0x9e3779b97f4a7c15ULL;
  z = *x;
  z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
  z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
  return z ^ (z >> 31);
}

void randombytes_seed(unsigned long long seed)
{
  unsigned long long x = seed ? seed : 0x9e3779b97f4a7c15ULL;

  randombytes_state[0] = splitmix64(&x);
  randombytes_state[1] = splitmix64(&x);
  randombytes_state[2] = splitmix64(&x);
  randombytes_state[3] = splitmix64(&x);
  randombytes_seeded = 1;
}

void randombytes_seed_bytes(const unsigned char *seed,long long seedlen)
{
  unsigned long long x = 0x9e3779b97f4a7c15ULL;

  while (seedlen-- > 0) {
    x ^= (unsigned long long) *seed++;
    x *= 0xbf58476d1ce4e5b9ULL;
    x ^= x >> 29;
  }

  randombytes_seed(x);
}

static void randombytes_deterministic(unsigned char *out,long long outlen)
{
  unsigned long long x;
  unsigned long long t;

  while (outlen-- > 0) {
    x = rotl(randombytes_state[1] * 5,7) * 9;
    t = randombytes_state[1] << 17;
    randombytes_state[2] ^= randombytes_state[0];
    randombytes_state[3] ^= randombytes_state[1];
    randombytes_state[1] ^= randombytes_state[2];
    randombytes_state[0] ^= randombytes_state[3];
    randombytes_state[2] ^= t;
    randombytes_state[3] = rotl(randombytes_state[3],45);
    *out++ = (unsigned char) (x >> 56);
  }
}

void randombytes(unsigned char *out,long long outlen)
{
  if (outlen <= 0) return;

  if (randombytes_seeded) {
    randombytes_seeded = 0;
    randombytes_deterministic(out,outlen);
    return;
  }

  if (__wasi_random_get((uint8_t *) out,(size_t) outlen) != 0) {
    __builtin_trap();
  }
}
