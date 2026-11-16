#pragma once
#include "common.hpp"
#include "knight.hpp"
#include "king.hpp"
#include "pawn.hpp"
#include "rook.hpp"
#include "bishop.hpp"
#include "queen.hpp"
#include "castling.hpp"
#include "promotion.hpp"
#include "enpassant.hpp"
#include "game_status.hpp"
#include "king_protection.hpp"

inline bool is_correct_turn(int piece_type, int side) {
    if (side == 0 && piece_type >= 6) return false;
    if (side == 1 && piece_type < 6) return false;
    return true;
}

inline bool is_legal_move(int piece_type, int from_square, int to_square, int enemy_king_square, 
                          U64 blockers, U64 friendly_pieces, CastlingRights castling_rights, 
                          int ep_square, int side, U64 bitboards[12]) {
    if (!is_correct_turn(piece_type, side)) return false;
    if (friendly_pieces & (1ULL << to_square)) return false;

    U64 attack_mask = 0ULL;
    int local_type = piece_type % 6;

    switch (local_type) {
        case 1: attack_mask = mask_knight_attacks(from_square); break;
        case 5: 
            attack_mask = mask_king_attacks(from_square);
            if (is_king_too_close(enemy_king_square, to_square)) return false;
            break;
        case 3: attack_mask = mask_rook_attacks(from_square, blockers); break;
        case 2: attack_mask = mask_bishop_attacks(from_square, blockers); break;
        case 4: attack_mask = mask_queen_attacks(from_square, blockers); break;
        case 0:
            {
                U64 attacks = mask_pawn_attacks(side, from_square);
                attack_mask = attacks;
                int from_rank = from_square / 8;
                int from_file = from_square % 8;
                int to_file = to_square % 8;
                
                if (from_file == to_file) {
                    if (side == 0) { // White Pawn
                        int single_step = from_square + 8;
                        // Single step forward
                        if (!(blockers & (1ULL << single_step))) {
                            if (single_step == to_square) {
                                attack_mask |= (1ULL << single_step);
                            }
                            // Initial two-square advance from Rank 2 (index 1)
                            if (from_rank == 1) {
                                int double_step = from_square + 16;
                                if (double_step == to_square && !(blockers & (1ULL << double_step))) {
                                    attack_mask |= (1ULL << double_step);
                                }
                            }
                        }
                    } else { // Black Pawn
                        int single_step = from_square - 8;
                        if (!(blockers & (1ULL << single_step))) {
                            if (single_step == to_square) {
                                attack_mask |= (1ULL << single_step);
                            }
                            if (from_rank == 6) {
                                int double_step = from_square - 16;
                                if (double_step == to_square && !(blockers & (1ULL << double_step))) {
                                    attack_mask |= (1ULL << double_step);
                                }
                            }
                        }
                    }
                }
                if (is_en_passant_valid(to_square, ep_square)) {
                    attack_mask |= (1ULL << ep_square);
                }
            }
            break;
    }

    if (!(attack_mask & (1ULL << to_square))) return false; 
    if (does_move_leave_king_in_check(from_square, to_square, side, bitboards, blockers)) return false;
    return true;
}
