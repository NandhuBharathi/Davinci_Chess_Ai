#pragma once
#include "common.hpp"

// Ethiri coin-ai vettu, bitboard-ai update panra function
inline void execute_capture(int to_square, U64 bitboards[12], int moving_side) {
    // moving_side: 0 = White (0 to 5 index), 1 = Black (6 to 11 index)
    int start_idx = (moving_side == 0) ? 6 : 7; // Enemy piece start index
    int end_idx = (moving_side == 0) ? 11 : 12;

    for (int i = start_idx; i <= end_idx; i++) {
        // Entha enemy bitboard-la intha destination square irukku nu thedi vetti azhikkrom!
        if (bitboards[i] & (1ULL << to_square)) {
            bitboards[i] &= ~(1ULL << to_square); // Bitboard-la irunthu coin-ai kaanamal akkurom (Vettiyachu!)
            break;
        }
    }
}
