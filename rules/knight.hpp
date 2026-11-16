#pragma once
#include "common.hpp"

inline U64 mask_knight_attacks(int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
    if ((bitboard >> 17) & NOT_H_FILE) attacks |= (bitboard >> 17);
    if ((bitboard >> 15) & NOT_A_FILE) attacks |= (bitboard >> 15);
    if ((bitboard >> 10) & NOT_GH_FILE) attacks |= (bitboard >> 10);
    if ((bitboard >> 6)  & NOT_AB_FILE) attacks |= (bitboard >> 6);
    if ((bitboard << 17) & NOT_A_FILE) attacks |= (bitboard << 17);
    if ((bitboard << 15) & NOT_H_FILE) attacks |= (bitboard << 15);
    if ((bitboard << 10) & NOT_AB_FILE) attacks |= (bitboard << 10);
    if ((bitboard << 6)  & NOT_GH_FILE) attacks |= (bitboard << 6);
    return attacks;
}
