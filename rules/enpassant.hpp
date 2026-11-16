#pragma once
#include "common.hpp"

// En Passant Target Square-ai check panra function
inline bool is_en_passant_valid(int target_square, int ep_square) {
    return (target_square == ep_square && ep_square != -1);
}
