#ifndef LIMB51_COMMON_H
#define LIMB51_COMMON_H

#include <stdint.h>
#include <string.h>

typedef unsigned __int128 uint128_t;

static inline uint64_t limb51_load64(const unsigned char *s)
{
    uint64_t value;
    memcpy(&value, s, sizeof value);
    return value;
}

static inline void limb51_store64(unsigned char *s, uint64_t value)
{
    memcpy(s, &value, sizeof value);
}

#define LOAD64_LE(s) limb51_load64(s)
#define STORE64_LE(s, value) limb51_store64(s, value)

#endif
