#include <iostream>
#include <string>
#include <cstdint>

typedef uint64_t U64;

// 12 Coins-kum Index (Caps = White, Small = Black)
enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

// Square-la coin-a ulla vaikkum function
void set_bit(U64 &board, int square) {
    board |= (1ULL << square);
}

// Neenga sonna Caps/Lowercase logic-a purinjikkum AI Parser!
void parse_fen(std::string fen) {
    int square = 56; // Top left corner (A8) irunthu start panrom
    
    for (char const &c : fen) {
        if (c == ' ') break; // Board layout mudinchuruchu
        if (c == '/') { square -= 16; continue; } // Adutha variykku (rank) pogum
        if (isdigit(c)) { square += (c - '0'); continue; } // Empty kattangal

        // Character-a vachi sariyaana bitboard-la set panrom
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

// Motha board-aiyum onna serthu print panna
void print_board() {
    std::cout << "\n  Davinci's Vision: The Full Board\n";
    std::cout << "  --------------------------------\n";
    for (int rank = 7; rank >= 0; rank--) {
        std::cout << rank + 1 << " | ";
        for (int file = 0; file < 8; file++) {
            int square = rank * 8 + file;
            char piece_char = '.'; // Empty aaga iruntha pulli (.)
            
            // Entha bitboard-la coin irukku nu thedi character-a maathuvathu
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
    // Chess-oda standard starting position FEN string
    std::string start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    
    parse_fen(start_fen);
    print_board();

    return 0;
}
