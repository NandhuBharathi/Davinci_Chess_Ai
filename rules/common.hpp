#pragma once
#include <cstdint>

typedef uint64_t U64;

// Board-ai vittu veliya pogaama thadukkum Masks
const U64 NOT_A_FILE  = 0xFEFEFEFEFEFEFEFEULL;
const U64 NOT_AB_FILE = 0xFCFCFCFCFCFCFCFCULL;
const U64 NOT_H_FILE  = 0x7F7F7F7F7F7F7F7FULL;
const U64 NOT_GH_FILE = 0x3F3F3F3F3F3F3F3FULL;

inline void set_bit(U64 &board, int square) {
    board |= (1ULL << square);
}
