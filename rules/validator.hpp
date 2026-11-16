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

// Turn validation: White turn-la Black piece-ai ota koodaathu, vice versa.
inline bool is_correct_turn(int piece_type, int side) {
    if (side == 0 && piece_type >= 6) return false; // White's turn (pieces 0-5), but black piece (6-11) selected!
    if (side == 1 && piece_type < 6) return false;  // Black's turn (pieces 6-11), but white piece (0-5) selected!
    return true;
}

// Absolute Legal Move Validation Function
inline bool is_legal_move(int piece_type, int from_square, int to_square, int enemy_king_square, 
                          U64 blockers, U64 friendly_pieces, CastlingRights castling_rights, 
                          int ep_square, int side, U64 bitboards[12]) {
    
    // 1. Turn Check
    if (!is_correct_turn(piece_type, side)) return false;

    // 2. Friendly Fire Check (Than voda own piece irukkura square-ku poga koodaathu)
    if (friendly_pieces & (1ULL << to_square)) return false;

    U64 attack_mask = 0ULL;
    int local_type = piece_type % 6; // 0:Pawn, 1:Knight, 2:Bishop, 3:Rook, 4:Queen, 5:King

    // 3. Piece Movement Rules Check
    switch (local_type) {
        case 1: // Knight
            attack_mask = mask_knight_attacks(from_square);
            break;
        case 5: // King
            attack_mask = mask_king_attacks(from_square);
            if (is_king_too_close(enemy_king_square, to_square)) return false;
            break;
        case 3: // Rook
            attack_mask = mask_rook_attacks(from_square, blockers);
            break;
        case 2: // Bishop
            attack_mask = mask_bishop_attacks(from_square, blockers);
            break;
        case 4: // Queen
            attack_mask = mask_queen_attacks(from_square, blockers);
            break;
        case 0: // Pawn
            {
                U64 attacks = mask_pawn_attacks(side, from_square);
                attack_mask = attacks;
                if (is_en_passant_valid(to_square, ep_square)) {
                    attack_mask |= (1ULL << ep_square);
                }
            }
            break;
    }

    // 4. Destination Square Attack Mask-kulla irukanum
    if (!(attack_mask & (1ULL << to_square))) {
        return false; 
    }

    // 5. King Protection Rule (Move senja aparam King check-la maattikuma nu simulation-la pakkarathu)
    if (does_move_leave_king_in_check(from_square, to_square, side, bitboards, blockers)) {
        return false; // King aabathula irukku, intha move illegal!
    }

    return true; // 100% Legal Move!
}
