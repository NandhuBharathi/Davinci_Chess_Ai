#pragma once
#include "common.hpp"

inline U64 mask_pawn_attacks(int side, int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
    if (side == 0) { // White
        if ((bitboard << 7) & NOT_H_FILE) attacks |= (bitboard << 7);
        if ((bitboard << 9) & NOT_A_FILE) attacks |= (bitboard << 9);
    } else { // Black
        if ((bitboard >> 7) & NOT_A_FILE) attacks |= (bitboard >> 7);
        if ((bitboard >> 9) & NOT_H_FILE) attacks |= (bitboard >> 9);
    }
    return attacks;
}
