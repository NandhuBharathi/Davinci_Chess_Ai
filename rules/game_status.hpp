#pragma once
#include "common.hpp"
#include "king.hpp"
#include "rook.hpp"
#include "bishop.hpp"
#include "queen.hpp"
#include "knight.hpp"
#include "pawn.hpp"

// 1. Check Detection: King square ethiriyoda attack-la irukkaa?
inline bool is_king_in_check(int king_square, U64 all_blockers, U64 enemy_pieces, 
                             const U64 enemy_bitboards[12], int enemy_side) {
    
    // Knight attacks
    U64 knights = enemy_bitboards[enemy_side == 0 ? 1 : 7]; 
    while (knights) {
        int sq = __builtin_ctzll(knights);
        if (mask_knight_attacks(sq) & (1ULL << king_square)) return true;
        knights &= knights - 1;
    }

    // Rook / Queen orthogonal attacks
    U64 rooks_queens = enemy_bitboards[enemy_side == 0 ? 3 : 9] | enemy_bitboards[enemy_side == 0 ? 4 : 10];
    while (rooks_queens) {
        int sq = __builtin_ctzll(rooks_queens);
        if (mask_rook_attacks(sq, all_blockers) & (1ULL << king_square)) return true;
        rooks_queens &= rooks_queens - 1;
    }

    // Bishop / Queen diagonal attacks
    U64 bishops_queens = enemy_bitboards[enemy_side == 0 ? 2 : 8] | enemy_bitboards[enemy_side == 0 ? 4 : 10];
    while (bishops_queens) {
        int sq = __builtin_ctzll(bishops_queens);
        if (mask_bishop_attacks(sq, all_blockers) & (1ULL << king_square)) return true;
        bishops_queens &= bishops_queens - 1;
    }

    // Pawn attacks
    U64 pawns = enemy_bitboards[enemy_side == 0 ? 0 : 6];
    while (pawns) {
        int sq = __builtin_ctzll(pawns);
        if (mask_pawn_attacks(enemy_side, sq) & (1ULL << king_square)) return true;
        pawns &= pawns - 1;
    }

    // King attacks
    int enemy_king_sq = __builtin_ctzll(enemy_bitboards[enemy_side == 0 ? 5 : 11]);
    if (mask_king_attacks(enemy_king_sq) & (1ULL << king_square)) return true;

    return false;
}

// 2. Game Result Outcomes Enum
enum GameResult {
    ONGOING = 0,
    WHITE_WIN_CHECKMATE = 1,
    BLACK_WIN_CHECKMATE = 2,
    DRAW_STALEMATE = 3,
    DRAW_INSUFFICIENT_MATERIAL = 4
};

// 3. Complete Game Outcome Evaluator
inline GameResult evaluate_game_state(U64 bitboards[12], int active_side, U64 all_blockers, bool has_any_legal_moves) {
    int white_king_sq = __builtin_ctzll(bitboards[5]);
    int black_king_sq = __builtin_ctzll(bitboards[11]);

    U64 white_pieces = bitboards[0] | bitboards[1] | bitboards[2] | bitboards[3] | bitboards[4] | bitboards[5];
    U64 black_pieces = bitboards[6] | bitboards[7] | bitboards[8] | bitboards[9] | bitboards[10] | bitboards[11];

    bool white_in_check = is_king_in_check(white_king_sq, all_blockers, black_pieces, bitboards, 1);
    bool black_in_check = is_king_in_check(black_king_sq, all_blockers, white_pieces, bitboards, 0);

    // Checkmate Check: Active side-kku check irukku, aana legal moves illai!
    if (active_side == 0 && white_in_check && !has_any_legal_moves) {
        return BLACK_WIN_CHECKMATE; // Black wins!
    }
    if (active_side == 1 && black_in_check && !has_any_legal_moves) {
        return WHITE_WIN_CHECKMATE; // White wins!
    }

    // Stalemate Check: Active side-kku check illai, aana move panna legal moves-e illai!
    if (active_side == 0 && !white_in_check && !has_any_legal_moves) {
        return DRAW_STALEMATE;
    }
    if (active_side == 1 && !black_in_check && !has_any_legal_moves) {
        return DRAW_STALEMATE;
    }

    return ONGOING; // Game continues!
}
