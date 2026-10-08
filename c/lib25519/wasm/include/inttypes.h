#ifndef WASM_INTTYPES_H
#define WASM_INTTYPES_H

/*
 * The compiler's inttypes.h defers to the libc one, which freestanding WebAssembly doesn't have.
 * cryptoint only needs the fixed-width integer types.
 */

#include <stdint.h>

#endif
