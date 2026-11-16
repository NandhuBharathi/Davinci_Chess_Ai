#include <iostream>
#include <string>
#include <cstdint>

typedef uint64_t U64;

enum Piece { P, N, B, R, Q, K, p, n, b, r, q, k };
U64 bitboards[12] = {0};

// File Masks (Kuthirai board-ai vittu veliye kuthikkama irukka)
const U64 NOT_A_FILE = 0xFEFEFEFEFEFEFEFEULL;
const U64 NOT_AB_FILE = 0xFCFCFCFCFCFCFCFCULL;
const U64 NOT_H_FILE = 0x7F7F7F7F7F7F7F7FULL;
const U64 NOT_GH_FILE = 0x3F3F3F3F3F3F3F3FULL;

void set_bit(U64 &board, int square) {
    board |= (1ULL << square);
}

// Knight-oda L-Shape moves-ai calculate panra Bitwise Magic
U64 mask_knight_attacks(int square) {
    U64 attacks = 0ULL;
    U64 bitboard = 0ULL;
    set_bit(bitboard, square);

    // Kuthiraiyin 8 disaigal (Bit shifts)
    if ((bitboard >> 17) & NOT_H_FILE) attacks |= (bitboard >> 17);
    if ((bitboard >> 15) & NOT_A_FILE) attacks |= (bitboard >> 15);
    if ((bitboard >> 10) & NOT_GH_FILE) attacks |= (bitboard >> 10);
    if ((bitboard >> 6)  & NOT_AB_FILE) attacks |= (bitboard >> 6);
    
    if ((bitboard << 17) & NOT_A_FILE) attacks |= (bitboard << 17);
    if ((bitboard << 15) & NOT_H_FILE) attacks |= (bitboard << 15);
    if ((bitboard << 10) & NOT_AB_FILE) attacks |= (bitboard << 10);
    if ((bitboard << 6)  & NOT_GH_FILE) attacks |= (bitboard << 6);

    return attacks;
}

// Attack map-a print panni paarkka
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
    // Example: D4 kattathil (Square 27) oru Knight irunthaal, athu enga ellam pogum?
    int knight_square = 27; 
    U64 knight_attacks = mask_knight_attacks(knight_square);
    
    print_bitboard(knight_attacks, "Davinci's Mind: Knight Attacks from D4");

    return 0;
}
