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

// Absolute Legal Move Validation Function
inline bool is_legal_move(int piece_type, int from_square, int to_square, int enemy_king_square, 
                          U64 blockers, U64 friendly_pieces, CastlingRights castling_rights, 
                          int ep_square, int side) {
    
    // Rule 1: Thannaoda orey piece irukkura kattathukku poga koodaathu (Friendly Fire)
    if (friendly_pieces & (1ULL << to_square)) return false;

    U64 attack_mask = 0ULL;

    // Piece-ku thevayana attack/move mask-ai fetch panrom
    switch (piece_type) {
        case 1: // Knight
            attack_mask = mask_knight_attacks(from_square);
            break;
        case 2: // King
            attack_mask = mask_king_attacks(from_square);
            // Rule 2: 2 King-um pakkathu pakkathula nerunga koodaathu!
            if (is_king_too_close(enemy_king_square, to_square)) return false;
            break;
        case 3: // Rook
            attack_mask = mask_rook_attacks(from_square, blockers);
            break;
        case 4: // Bishop
            attack_mask = mask_bishop_attacks(from_square, blockers);
            break;
        case 5: // Queen
            attack_mask = mask_queen_attacks(from_square, blockers);
            break;
        case 6: // Pawn
            {
                U64 attacks = mask_pawn_attacks(side, from_square);
                U64 pushes = 0ULL; // (Pushes logic can be appended based on empty squares)
                attack_mask = attacks | pushes;
                // En Passant check
                if (is_en_passant_valid(to_square, ep_square)) {
                    attack_mask |= (1ULL << ep_square);
                }
            }
            break;
    }

    // Rule 3: Calculated attack mask-kulla destination square irukanum!
    if (!(attack_mask & (1ULL << to_square))) {
        return false; // Intha move andha piece-ku illegal!
    }

    return true; // Ella rules-um pass aagiruchu! Move is 100% Legal!
}
