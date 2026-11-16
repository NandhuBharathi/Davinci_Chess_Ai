#pragma once
#include "common.hpp"

// Castling Rights struct (Raja or Rook move aanal false aagum)
struct CastlingRights {
    bool white_kingside = true;
    bool white_queenside = true;
    bool black_kingside = true;
    bool black_queenside = true;
};

// Complete FIDE Castling Validation Rule
inline bool can_castle(int side, char type, CastlingRights rights, U64 all_occupancies, bool king_in_check, bool path_under_attack) {
    // 1. Raja check-la irunthaal evlavu kootinaalum castling panna mudiyaathu!
    if (king_in_check) return false;
    
    // 2. Raja cross panra vazhi (path) ethiriyin attack-la irunthaalum illegal!
    if (path_under_attack) return false;

    if (side == 0) { // White
        if (type == 'K') { // Kingside (E1 to G1)
            if (!rights.white_kingside) return false; // Raja/Rook munnadi move aagirukku
            // F1 (Square 5) mariyum G1 (Square 6) empty-aaga irukanum
            if (all_occupancies & ((1ULL << 5) | (1ULL << 6))) return false;
        } 
        else if (type == 'Q') { // Queenside (E1 to C1)
            if (!rights.white_queenside) return false;
            // B1 (Square 1), C1 (Square 2), D1 (Square 3) empty-aaga irukanum
            if (all_occupancies & ((1ULL << 1) | (1ULL << 2) | (1ULL << 3))) return false;
        }
    } 
    else { // Black
        if (type == 'k') { // Black Kingside (E8 to G8)
            if (!rights.black_kingside) return false;
            if (all_occupancies & ((1ULL << 61) | (1ULL << 62))) return false;
        } 
        else if (type == 'q') { // Black Queenside (E8 to C8)
            if (!rights.black_queenside) return false;
            if (all_occupancies & ((1ULL << 57) | (1ULL << 58) | (1ULL << 59))) return false;
        }
    }
    return true;
}
