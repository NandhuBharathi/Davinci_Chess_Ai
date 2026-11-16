#pragma once
#include "common.hpp"
#include "rook.hpp"
#include "bishop.hpp"

// Raniyin moves = Yaanai + Manthiri
inline U64 mask_queen_attacks(int square, U64 blockers) {
    return mask_rook_attacks(square, blockers) | mask_bishop_attacks(square, blockers);
}
