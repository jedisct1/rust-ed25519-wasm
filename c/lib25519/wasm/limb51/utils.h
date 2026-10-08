#ifndef LIMB51_UTILS_H
#define LIMB51_UTILS_H

#include <stddef.h>
#include "crypto_verify_32.h"

static inline int sodium_is_zero(const unsigned char *s, size_t length)
{
    static const unsigned char zero[32] = {0};
    (void) length;
    return crypto_verify_32(s, zero) == 0;
}

#endif
