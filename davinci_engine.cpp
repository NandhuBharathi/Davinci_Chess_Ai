#include <iostream>
#include <string>
#include "rules/common.hpp"
#include "rules/knight.hpp"
#include "rules/king.hpp"
#include "rules/pawn.hpp"
#include "rules/rook.hpp" // Rook rule-a inaikkurom!

enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

void parse_fen(std::string fen) {
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

// Bitboard printing-kaga oru chinna helper function
void print_bitboard(U64 board, std::string title) {
    std::cout << "\n  " << title << "\n";
    std::cout << "  --------------------------------\n";
    for (int rank = 7; rank >= 0; rank--) {
        std::cout << rank + 1 << " | ";
        for (int file = 0; file < 8; file++) {
            int square = rank * 8 + file;
            std::cout << ((board & (1ULL << square)) ? "X " : ". ");
        }
        std::cout << "\n";
    }
    std::cout << "    ----------------\n";
    std::cout << "    A B C D E F G H\n\n";
}

int main() {
    std::cout << "🔥 Davinci Engine: Rook Ray Casting Active 🔥\n";
    
    // Test Scenario: E4-la oru Yaanai (Square 28). 
    // E6-la oru Sipaai irukku (Blocker). 
    // Yaanai E6-a thaandi poga koodathu.
    
    U64 blockers = 0ULL;
    set_bit(blockers, 44); // E6 kattathula oru coin nirkuthu
    
    U64 rook_attacks = mask_rook_attacks(28, blockers);
    
    print_bitboard(blockers, "Board-la ulla Blockers (E6)");
    print_bitboard(rook_attacks, "Rook Attacks (E4-la irunthu)");

    return 0;
}
