#pragma once
#include "common.hpp"

inline U64 mask_king_attacks(int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
    if (bitboard >> 8) attacks |= (bitboard >> 8); 
    if ((bitboard >> 9) & NOT_H_FILE) attacks |= (bitboard >> 9); 
    if ((bitboard >> 7) & NOT_A_FILE) attacks |= (bitboard >> 7); 
    if ((bitboard >> 1) & NOT_H_FILE) attacks |= (bitboard >> 1); 
    if (bitboard << 8) attacks |= (bitboard << 8); 
    if ((bitboard << 9) & NOT_A_FILE) attacks |= (bitboard << 9); 
    if ((bitboard << 7) & NOT_H_FILE) attacks |= (bitboard << 7); 
    if ((bitboard << 1) & NOT_A_FILE) attacks |= (bitboard << 1); 
    return attacks;
}
