#pragma once
#include "common.hpp"
#include <cmath>
#include <algorithm>

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

class PawnRules {
public:
    /**
     * Validates pawn movement and captures according to FIDE rules.
     * side: 0 for White, 1 for Black
     * is_capture: boolean indicating if an opponent piece is on the target square
     */
    static bool validate(int from_sq, int to_sq, int side, bool is_capture) {
        int from_rank = from_sq / 8;
        int from_file = from_sq % 8;
        int to_rank = to_sq / 8;
        int to_file = to_sq % 8;
        
        int rank_diff = to_rank - from_rank;
        int file_diff = std::abs(to_file - from_file);

        if (side == 0) { // White Pawn (Moves UP: Rank 2 -> 8, i.e., index 1 to 7)
            // 1. Single square forward advance (No capture)
            if (file_diff == 0 && rank_diff == 1 && !is_capture) {
                return true;
            }
            
            // 2. Initial two-square advance from Rank 2 (index 1)
            if (file_diff == 0 && from_rank == 1 && rank_diff == 2 && !is_capture) {
                return true;
            }
            
            // 3. Diagonal capture (Must capture an opponent piece)
            if (file_diff == 1 && rank_diff == 1 && is_capture) {
                return true;
            }
            
        } else { // Black Pawn (Moves DOWN: Rank 7 -> Rank 1, i.e., index 6 to 0)
            // 1. Single square forward advance (No capture)
            if (file_diff == 0 && rank_diff == -1 && !is_capture) {
                return true;
            }
            
            // 2. Initial two-square advance from Rank 7 (index 6)
            if (file_diff == 0 && from_rank == 6 && rank_diff == -2 && !is_capture) {
                return true;
            }
            
            // 3. Diagonal capture (Must capture an opponent piece)
            if (file_diff == 1 && rank_diff == -1 && is_capture) {
                return true;
            }
        }
        
        return false;
    }
};
