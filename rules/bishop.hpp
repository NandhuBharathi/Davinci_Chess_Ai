#pragma once
#include "common.hpp"

// Manthiriyin Cross (X) moves
inline U64 mask_bishop_attacks(int square, U64 blockers) {
    U64 attacks = 0ULL;
    int r = square / 8;
    int f = square % 8;
    
    // 1. North-East (Mela-Right)
    for (int i = r + 1, j = f + 1; i <= 7 && j <= 7; i++, j++) {
        U64 sq_bit = (1ULL << (i * 8 + j));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    // 2. North-West (Mela-Left)
    for (int i = r + 1, j = f - 1; i <= 7 && j >= 0; i++, j--) {
        U64 sq_bit = (1ULL << (i * 8 + j));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    // 3. South-East (Keezha-Right)
    for (int i = r - 1, j = f + 1; i >= 0 && j <= 7; i--, j++) {
        U64 sq_bit = (1ULL << (i * 8 + j));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    // 4. South-West (Keezha-Left)
    for (int i = r - 1, j = f - 1; i >= 0 && j >= 0; i--, j--) {
        U64 sq_bit = (1ULL << (i * 8 + j));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    return attacks;
}
