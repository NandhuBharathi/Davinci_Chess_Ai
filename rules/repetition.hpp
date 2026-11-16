#pragma once
#include "common.hpp"
#include <unordered_map>

class RepetitionTracker {
private:
    std::unordered_map<uint64_t, int> position_history;

public:
    // Simple hash generator for board state tracking
    uint64_t compute_hash(const U64 bitboards[12], int active_side) {
        uint64_t hash = 0;
        for (int i = 0; i < 12; i++) {
            U64 bb = bitboards[i];
            while (bb) {
                int sq = __builtin_ctzll(bb);
                hash ^= (sq + 1) * (i + 1) * 0x9e3779b97f4a7c15ULL;
                bb &= bb - 1;
            }
        }
        hash ^= (active_side == 0 ? 0x12345678abcdef00ULL : 0xfedcba0987654321ULL);
        return hash;
    }

    void record_position(uint64_t hash) {
        position_history[hash]++;
    }

    void clear() {
        position_history.clear();
    }

    bool is_threefold_repetition(uint64_t hash) {
        if (position_history.find(hash) != position_history.end()) {
            return position_history[hash] >= 3;
        }
        return false;
    }
};
