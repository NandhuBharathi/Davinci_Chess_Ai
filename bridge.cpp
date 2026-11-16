#include <iostream>
#include <string>
#include <cstdint>

// Davinci Engine Core Headers Inclusion
#include "rules/validator.hpp"
#include "rules/game_status.hpp"
#include "rules/draw_rules.hpp"
#include "rules/repetition.hpp"
#include "rules/special_rules.hpp"

// Global Bitboards for Davinci Engine Bridge
extern "C" {
    U64 davinci_bitboards[12] = {0};
    CastlingRights davinci_rights;

    // 1. Davinci Engine FEN Board Initializer
    void davinci_set_fen(const char* fen_str) {
        std::string fen(fen_str);
        for (int i = 0; i < 12; i++) davinci_bitboards[i] = 0;
        int square = 56; 
        
        for (char const &c : fen) {
            if (c == ' ') break; 
            if (c == '/') { square -= 16; continue; } 
            if (isdigit(c)) { square += (c - '0'); continue; } 

            switch (c) {
                case 'P': set_bit(davinci_bitboards[0], square); break;
                case 'N': set_bit(davinci_bitboards[1], square); break;
                case 'B': set_bit(davinci_bitboards[2], square); break;
                case 'R': set_bit(davinci_bitboards[3], square); break;
                case 'Q': set_bit(davinci_bitboards[4], square); break;
                case 'K': set_bit(davinci_bitboards[5], square); break;
                case 'p': set_bit(davinci_bitboards[6], square); break;
                case 'n': set_bit(davinci_bitboards[7], square); break;
                case 'b': set_bit(davinci_bitboards[8], square); break;
                case 'r': set_bit(davinci_bitboards[9], square); break;
                case 'q': set_bit(davinci_bitboards[10], square); break;
                case 'k': set_bit(davinci_bitboards[11], square); break;
            }
            square++;
        }
    }

    // 2. Davinci Engine Move Validation Function
    bool davinci_validate_move(int piece_type, int from_sq, int to_sq, int side) {
        U64 all_blockers = 0ULL;
        for(int i = 0; i < 12; i++) all_blockers |= davinci_bitboards[i];
        
        U64 white_pieces = davinci_bitboards[0] | davinci_bitboards[1] | davinci_bitboards[2] | 
                           davinci_bitboards[3] | davinci_bitboards[4] | davinci_bitboards[5];
        
        int enemy_king_sq = __builtin_ctzll(davinci_bitboards[side == 0 ? 11 : 5]);

        return is_legal_move(piece_type, from_sq, to_sq, enemy_king_sq, all_blockers, 
                             white_pieces, davinci_rights, -1, side, davinci_bitboards);
    }
}
