#pragma once
#include "common.hpp"

// Yaanaiyin moves-ai uruvaakkum function. 
// 'blockers' ngurathu board-la ulla motha coins-oda bitboard.
inline U64 mask_rook_attacks(int square, U64 blockers) {
    U64 attacks = 0ULL;
    
    // Square-oda row (rank) mariyum column (file) kandupudikkirom
    int r = square / 8;
    int f = square % 8;
    
    // 1. Mela pogum (North)
    for (int i = r + 1; i <= 7; i++) {
        U64 sq_bit = (1ULL << (i * 8 + f));
        attacks |= sq_bit;
        if (sq_bit & blockers) break; // Kattam block aagirunthaa ninnudum!
    }
    
    // 2. Keezha pogum (South)
    for (int i = r - 1; i >= 0; i--) {
        U64 sq_bit = (1ULL << (i * 8 + f));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    
    // 3. Right-la pogum (East)
    for (int i = f + 1; i <= 7; i++) {
        U64 sq_bit = (1ULL << (r * 8 + i));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    
    // 4. Left-la pogum (West)
    for (int i = f - 1; i >= 0; i--) {
        U64 sq_bit = (1ULL << (r * 8 + i));
        attacks |= sq_bit;
        if (sq_bit & blockers) break;
    }
    
    return attacks;
}
