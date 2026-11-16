#pragma once
#include "common.hpp"

// Raja (King) Moves - Suthi ulla 8 kattangal
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

// 2 King-um pakkathu pakkathula nerunga mudiyaathu rule check!
inline bool is_king_too_close(int enemy_king_square, int my_king_target_square) {
    // Enemy King-oda attack zones-kulla namma King pona athu illegal move!
    U64 enemy_king_zone = mask_king_attacks(enemy_king_square);
    if (enemy_king_zone & (1ULL << my_king_target_square)) {
        return true; // Aahaa, rendu King-um orey nerathula pakkathula varraanga! Illegal!
    }
    return false;
}
