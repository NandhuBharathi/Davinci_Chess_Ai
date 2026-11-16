#include <iostream>
#include <string>
#include <cstdint>
#include "rules/validator.hpp"

// 12 Coins Index
enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

// FEN Parser
void parse_fen(std::string fen) {
    for (int i = 0; i < 12; i++) bitboards[i] = 0;
    int square = 56; 
    
    for (char const &c : fen) {
        if (c == ' ') break; 
        if (c == '/') { square -= 16; continue; } 
        if (isdigit(c)) { square += (c - '0'); continue; } 

        switch (c) {
            case 'P': set_bit(bitboards[P], square); break;
            case 'N': set_bit(bitboards[N], square); break;
            case 'B': set_bit(bitboards[B], square); break;
            case 'R': set_bit(bitboards[R], square); break;
            case 'Q': set_bit(bitboards[Q], square); break;
            case 'K': set_bit(bitboards[K], square); break;
            case 'p': set_bit(bitboards[p], square); break;
            case 'n': set_bit(bitboards[n], square); break;
            case 'b': set_bit(bitboards[b], square); break;
            case 'r': set_bit(bitboards[r], square); break;
            case 'q': set_bit(bitboards[q], square); break;
            case 'k': set_bit(bitboards[k], square); break;
        }
        square++;
    }
}

void print_board() {
    std::cout << "\n  Davinci's Vision: The Full Board\n";
    std::cout << "  --------------------------------\n";
    for (int rank = 7; rank >= 0; rank--) {
        std::cout << rank + 1 << " | ";
        for (int file = 0; file < 8; file++) {
            int square = rank * 8 + file;
            char piece_char = '.'; 
            
            if (bitboards[P] & (1ULL << square)) piece_char = 'P';
            else if (bitboards[N] & (1ULL << square)) piece_char = 'N';
            else if (bitboards[B] & (1ULL << square)) piece_char = 'B';
            else if (bitboards[R] & (1ULL << square)) piece_char = 'R';
            else if (bitboards[Q] & (1ULL << square)) piece_char = 'Q';
            else if (bitboards[K] & (1ULL << square)) piece_char = 'K';
            else if (bitboards[p] & (1ULL << square)) piece_char = 'p';
            else if (bitboards[n] & (1ULL << square)) piece_char = 'n';
            else if (bitboards[b] & (1ULL << square)) piece_char = 'b';
            else if (bitboards[r] & (1ULL << square)) piece_char = 'r';
            else if (bitboards[q] & (1ULL << square)) piece_char = 'q';
            else if (bitboards[k] & (1ULL << square)) piece_char = 'k';

            std::cout << piece_char << " ";
        }
        std::cout << "\n";
    }
    std::cout << "    ----------------\n";
    std::cout << "    A B C D E F G H\n\n";
}

int main() {
    std::cout << "🔥 Davinci Engine: Strict FIDE Rule Validator Active 🔥\n\n";
    
    parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    print_board();

    // Testing Move Legality Rule Check:
    // Let's test if a Knight from B1 (Square 1) can legally move to C3 (Square 18)
    U64 all_blockers = 0ULL;
    for(int i=0; i<12; i++) all_blockers |= bitboards[i];
    U64 white_pieces = bitboards[P] | bitboards[N] | bitboards[B] | bitboards[R] | bitboards[Q] | bitboards[K];
    CastlingRights rights;

    bool test_move = is_legal_move(1, 1, 18, 60, all_blockers, white_pieces, rights, -1, 0);
    
    std::cout << "Move Test (Knight B1 -> C3): " << (test_move ? "LEGAL ✅" : "ILLEGAL ❌") << "\n";

    return 0;
}
