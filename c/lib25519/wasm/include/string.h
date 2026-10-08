#ifndef WASM_STRING_H
#define WASM_STRING_H

#include <stddef.h>

#define memcpy(A, B, C) __builtin_memcpy((A), (B), (C))
#define memmove(A, B, C) __builtin_memmove((A), (B), (C))
#define memset(A, B, C) __builtin_memset((A), (B), (C))

#endif
