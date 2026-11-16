#include <iostream>
#include <string>
#include <cstdint>

// Modular FIDE Rules Headers
#include "rules/validator.hpp"
#include "rules/game_status.hpp"
#include "rules/draw_rules.hpp"
#include "rules/repetition.hpp"
#include "rules/special_rules.hpp"

// 12 Coins Index (0-5: White [P, N, B, R, Q, K], 6-11: Black [p, n, b, r, q, k])
enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

// FEN Parser: Board state-ai initialize panra function
void parse_fen(std::string fen) {
    for (int i = 0; i < 12; i++) bitboards[i] = 0;
    int square = 56; // Start from A8 (Rank 8, File A)
    
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

// Full Board ASCII Visualizer
void print_board() {
    std::cout << "\n  🔥 Davinci Engine: Board State 🔥\n";
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
    std::cout << "🚀 Davinci Chess Engine Core Initialized Successfully.\n";
    
    // Load standard starting FEN position
    parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
    // Display board vision
    print_board();

    return 0;
}
