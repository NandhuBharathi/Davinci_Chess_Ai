use std::fmt;

// ============================================================
// 1. COLOR REGISTRY
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

// ============================================================
// 2. PIECE REGISTRY
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::King,
        Piece::Queen,
        Piece::Rook,
        Piece::Bishop,
        Piece::Knight,
        Piece::Pawn,
    ];

    pub fn points(self) -> u16 {
        match self {
            Piece::King => 0,
            Piece::Queen => 9,
            Piece::Rook => 5,
            Piece::Bishop => 3,
            Piece::Knight => 3,
            Piece::Pawn => 1,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Piece::King => 'K',
            Piece::Queen => 'Q',
            Piece::Rook => 'R',
            Piece::Bishop => 'B',
            Piece::Knight => 'N',
            Piece::Pawn => 'P',
        }
    }

    pub fn starting_count(self) -> u32 {
        match self {
            Piece::King | Piece::Queen => 1,
            Piece::Rook | Piece::Bishop | Piece::Knight => 2,
            Piece::Pawn => 8,
        }
    }
}

// ============================================================
// 3. SQUARE SYSTEM
// A1 = 0, H8 = 63
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub fn new(index: u8) -> Option<Self> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    pub fn file(self) -> u8 {
        self.0 % 8
    }

    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    pub fn bit(self) -> u64 {
        1u64 << self.0
    }

    pub fn from_coords(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }

    pub fn name(self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;

        format!("{}{}", file, rank)
    }
}

// ============================================================
// 4. BITBOARD SYSTEM
// ============================================================

#[derive(Debug, Clone, Copy, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn set(&mut self, square: Square) {
        self.0 |= square.bit();
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !square.bit();
    }

    pub fn contains(self, square: Square) -> bool {
        (self.0 & square.bit()) != 0
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn pop_lsb(&mut self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }

        let index = self.0.trailing_zeros() as u8;

        self.0 &= self.0 - 1;

        Some(Square(index))
    }
}

// ============================================================
// 5. CHESS BOARD
// ============================================================

#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub pieces: [[Bitboard; 6]; 2],

    pub occupied: [Bitboard; 2],

    pub all_occupied: Bitboard,

    pub side_to_move: Color,
}

impl ChessBoard {
    pub fn empty() -> Self {
        Self {
            pieces: [[Bitboard(0); 6]; 2],
            occupied: [Bitboard(0); 2],
            all_occupied: Bitboard(0),
            side_to_move: Color::White,
        }
    }

    pub fn add_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[color.index()][piece as usize].set(square);

        self.rebuild_occupancy();
    }

    pub fn rebuild_occupancy(&mut self) {
        self.occupied = [Bitboard(0); 2];

        for color in 0..2 {
            for piece in 0..6 {
                self.occupied[color].0 |= self.pieces[color][piece].0;
            }
        }

        self.all_occupied.0 = self.occupied[0].0 | self.occupied[1].0;
    }

    pub fn piece_count(&self, color: Color, piece: Piece) -> u32 {
        self.pieces[color.index()][piece as usize].count()
    }

    pub fn total_pieces(&self) -> u32 {
        self.all_occupied.count()
    }

    pub fn initial_position() -> Self {
        let mut board = Self::empty();

        let back_rank = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for file in 0..8 {
            // White back rank
            board.add_piece(Color::White, back_rank[file], Square(file as u8));

            // White pawns
            board.add_piece(Color::White, Piece::Pawn, Square(8 + file as u8));

            // Black pawns
            board.add_piece(Color::Black, Piece::Pawn, Square(48 + file as u8));

            // Black back rank
            board.add_piece(Color::Black, back_rank[file], Square(56 + file as u8));
        }

        board
    }

    pub fn material_points(&self, color: Color) -> u32 {
        let mut total = 0;

        for piece in Piece::ALL {
            total += self.piece_count(color, piece) * piece.points() as u32;
        }

        total
    }

    pub fn validate(&self) -> bool {
        // White and Black cannot occupy same square.
        if self.occupied[0].0 & self.occupied[1].0 != 0 {
            return false;
        }

        // Occupancy consistency.
        if self.all_occupied.0 != (self.occupied[0].0 | self.occupied[1].0) {
            return false;
        }

        // Exactly one King per side.
        for color in [Color::White, Color::Black] {
            if self.piece_count(color, Piece::King) != 1 {
                return false;
            }
        }

        true
    }
}

// ============================================================
// 6. BOARD DISPLAY
// ============================================================

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;

            for file in 0..8 {
                let square = Square(rank * 8 + file);

                let mut symbol = '.';

                for color in [Color::White, Color::Black] {
                    for piece in Piece::ALL {
                        if self.pieces[color.index()][piece as usize].contains(square) {
                            symbol = piece.symbol();

                            if color == Color::Black {
                                symbol = symbol.to_ascii_lowercase();
                            }
                        }
                    }
                }

                write!(f, "{} ", symbol)?;
            }

            writeln!(f)?;
        }

        writeln!(f, "  a b c d e f g h")
    }
}

// ============================================================
// 7. UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_initial_piece_count() {
        let board = ChessBoard::initial_position();

        assert_eq!(board.total_pieces(), 32);
    }

    #[test]
    fn test_white_pieces() {
        let board = ChessBoard::initial_position();

        for piece in Piece::ALL {
            assert_eq!(
                board.piece_count(Color::White, piece),
                piece.starting_count()
            );
        }
    }

    #[test]
    fn test_black_pieces() {
        let board = ChessBoard::initial_position();

        for piece in Piece::ALL {
            assert_eq!(
                board.piece_count(Color::Black, piece),
                piece.starting_count()
            );
        }
    }

    #[test]
    fn test_initial_material() {
        let board = ChessBoard::initial_position();

        assert_eq!(board.material_points(Color::White), 39);
        assert_eq!(board.material_points(Color::Black), 39);
    }

    #[test]
    fn test_board_validity() {
        let board = ChessBoard::initial_position();

        assert!(board.validate());
    }

    #[test]
    fn test_square_mapping() {
        assert_eq!(Square(0).name(), "a1");
        assert_eq!(Square(7).name(), "h1");
        assert_eq!(Square(56).name(), "a8");
        assert_eq!(Square(63).name(), "h8");
    }

    #[test]
    fn test_bitboard() {
        let mut bb = Bitboard::default();

        let square = Square(4);

        bb.set(square);

        assert!(bb.contains(square));
        assert_eq!(bb.count(), 1);

        bb.clear(square);

        assert!(!bb.contains(square));
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = Bitboard(0b1010);

        assert_eq!(bb.pop_lsb(), Some(Square(1)));
        assert_eq!(bb.pop_lsb(), Some(Square(3)));
        assert_eq!(bb.pop_lsb(), None);
    }
}

pub mod move_generator;

pub mod legal_move;

pub mod game_state;

pub mod special_moves;

pub mod en_passant;

pub mod move_executor;

pub mod chess_action;

pub mod action_encoding;

pub mod legal_action_mask;

pub mod position_evaluation;

pub mod king_safety;

pub mod pawn_structure;

pub mod pawn_race;

pub mod unified_evaluation;

pub mod unified_evaluation_v2;

pub mod terminal_state;

pub mod terminal_state_v2;

pub mod game_history;

pub mod state_manager;

pub mod strategy_discovery;

pub mod board_encoder;

pub mod neural_network;

pub mod strategic_integration;

pub mod brain;
