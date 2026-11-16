#pragma once
#include "common.hpp"

// Sipaai promotion check panra function
inline bool is_pawn_promotion(int side, int square) {
    int rank = square / 8;
    if (side == 0 && rank == 7) return true; // White reaches Rank 8
    if (side == 1 && rank == 0) return true; // Black reaches Rank 1
    if (side == 1 && rank == 7) return true; // (Safety check based on board orientation)
    return false;
}
