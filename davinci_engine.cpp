#include <iostream>
#include <string>
#include "rules/common.hpp"
#include "rules/knight.hpp"
#include "rules/king.hpp"
#include "rules/pawn.hpp"
#include "rules/rook.hpp"
#include "rules/bishop.hpp"
#include "rules/queen.hpp" 

enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

void parse_fen(std::string fen) { /* Pazhaya code apdiye irukkum, space-kaga omit panren ulla irukku nu nenaichukkonga */ }

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
    std::cout << "🔥 Davinci Engine: Bishop & Queen Active 🔥\n";
    
    // D4 (Square 27) la ulla Rani (Queen) enga ellam pogum nu test panrom
    // Blocker ethuvum illama full empty board (0ULL) kudukkurom
    U64 empty_blockers = 0ULL;
    
    U64 queen_attacks = mask_queen_attacks(27, empty_blockers);
    print_bitboard(queen_attacks, "Queen Attacks (D4-la irunthu - Star Shape)");

    return 0;
}
