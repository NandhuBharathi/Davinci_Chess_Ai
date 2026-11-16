#pragma once
#include "common.hpp"
#include "game_status.hpp"

// Check-la irukkum pothu, King-ai save panra allathu check-ai thadukkra validation!
inline bool does_move_leave_king_in_check(int from_square, int to_square, int side, 
                                          U64 bitboards[12], U64 all_blockers) {
    // 1. Temporary-aa board state-ai copy panrom (Simulation)
    U64 temp_bitboards[12];
    for(int i=0; i<12; i++) temp_bitboards[i] = bitboards[i];

    // 2. Simulated Move-ai execute panrom
    // (Moving piece-ai from_square-la irunthu to_square-ku mathrom)
    for(int i = (side == 0 ? 0 : 6); i <= (side == 0 ? 5 : 11); i++) {
        if (temp_bitboards[i] & (1ULL << from_square)) {
            temp_bitboards[i] &= ~(1ULL << from_square);
            temp_bitboards[i] |= (1ULL << to_square);
            break;
        }
    }

    // Simulation-kku new blockers calculate panrom
    U64 temp_blockers = 0ULL;
    for(int i=0; i<12; i++) temp_blockers |= temp_bitboards[i];

    // 3. Intha move pannuna piragu, namma King innum check-la irukkaana nu paarkrom!
    int my_king_square = __builtin_ctzll(temp_bitboards[side == 0 ? 5 : 11]); // White King (5) or Black King (11)
    int enemy_side = (side == 0) ? 1 : 0;
    U64 enemy_pieces = 0ULL;
    for(int i = (enemy_side == 0 ? 0 : 6); i <= (enemy_side == 0 ? 5 : 11); i++) {
        enemy_pieces |= temp_bitboards[i];
    }

    // Oru vela intha move pannunaal King-ku check vizhunthaal, antha move 100% ILLEGAL!
    if (is_king_in_check(my_king_square, temp_blockers, enemy_pieces, temp_bitboards, enemy_side)) {
        return true; // Yes, King innum check-la maattikuchu! This move is banned!
    }

    return false; // Safe! Move can be played.
}
