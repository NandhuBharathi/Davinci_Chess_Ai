#pragma once
#include "common.hpp"
#include "draw_rules.hpp"
#include "repetition.hpp"

// 1. 50-Move Rule Checker
inline bool is_fifty_move_draw(int halfmove_clock) {
    // Chess rule: 50 moves (100 half-moves) without pawn push or capture = automatic draw
    return (halfmove_clock >= 100);
}

// 2. Insufficient Material Checker
inline bool is_insufficient_material(const U64 bitboards[12]) {
    // Count total pieces on board
    int total_pieces = 0;
    for (int i = 0; i < 12; i++) {
        U64 bb = bitboards[i];
        while (bb) {
            total_pieces++;
            bb &= bb - 1;
        }
    }

    // King vs King (Only 2 kings on board)
    if (total_pieces == 2) return true;

    // King + Knight vs King OR King + Bishop vs King (Only 3 pieces)
    if (total_pieces == 3) {
        // White or Black has a Knight or Bishop
        if (bitboards[1] || bitboards[7] || bitboards[2] || bitboards[8]) {
            return true;
        }
    }

    return false;
}
