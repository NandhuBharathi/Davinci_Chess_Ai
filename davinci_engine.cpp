#include <iostream>
#include <string>
#include <cstdint>

typedef uint64_t U64;

// File Masks (Board-a vittu veliya pogaama thadukka)
const U64 NOT_A_FILE = 0xFEFEFEFEFEFEFEFEULL;
const U64 NOT_H_FILE = 0x7F7F7F7F7F7F7F7FULL;
const U64 NOT_AB_FILE = 0xFCFCFCFCFCFCFCFCULL;
const U64 NOT_GH_FILE = 0x3F3F3F3F3F3F3F3FULL;

void set_bit(U64 &board, int square) { board |= (1ULL << square); }

// 1. Kuthirai (Knight) Moves
U64 mask_knight_attacks(int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
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

// 2. Raja (King) Moves - Suthi ulla 8 kattangal
U64 mask_king_attacks(int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
    if (bitboard >> 8) attacks |= (bitboard >> 8); // Keezhe
    if ((bitboard >> 9) & NOT_H_FILE) attacks |= (bitboard >> 9); // Keezhe-Right
    if ((bitboard >> 7) & NOT_A_FILE) attacks |= (bitboard >> 7); // Keezhe-Left
    if ((bitboard >> 1) & NOT_H_FILE) attacks |= (bitboard >> 1); // Right

    if (bitboard << 8) attacks |= (bitboard << 8); // Mele
    if ((bitboard << 9) & NOT_A_FILE) attacks |= (bitboard << 9); // Mele-Left
    if ((bitboard << 7) & NOT_H_FILE) attacks |= (bitboard << 7); // Mele-Right
    if ((bitboard << 1) & NOT_A_FILE) attacks |= (bitboard << 1); // Left
    return attacks;
}

// 3. Sipaai (Pawn) Attacks - Cross-aaga vettum idangal
U64 mask_pawn_attacks(int side, int square) {
    U64 attacks = 0ULL, bitboard = 0ULL; set_bit(bitboard, square);
    // side 0 = White (Mela pogum), side 1 = Black (Keezha pogum)
    if (side == 0) { 
        if ((bitboard << 7) & NOT_H_FILE) attacks |= (bitboard << 7);
        if ((bitboard << 9) & NOT_A_FILE) attacks |= (bitboard << 9);
    } else {
        if ((bitboard >> 7) & NOT_A_FILE) attacks |= (bitboard >> 7);
        if ((bitboard >> 9) & NOT_H_FILE) attacks |= (bitboard >> 9);
    }
    return attacks;
}

// Map-a print panna
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
    // Test 1: Raja E4-la (Square 28) irunthaal
    U64 king_attacks = mask_king_attacks(28);
    print_bitboard(king_attacks, "Davinci's Mind: King Attacks from E4");

    // Test 2: White Pawn D2-la (Square 11) irunthaal ethai vettum?
    U64 white_pawn_attacks = mask_pawn_attacks(0, 11);
    print_bitboard(white_pawn_attacks, "Davinci's Mind: White Pawn Attacks from D2");

    return 0;
}
