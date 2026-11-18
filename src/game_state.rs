use crate::{Color, Square};
use serde::{Deserialize, Serialize};

// ======================================================
// CASTLING RIGHTS
// ======================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl CastlingRights {
    pub fn disable_king(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_kingside = false;
                self.white_queenside = false;
            }
            Color::Black => {
                self.black_kingside = false;
                self.black_queenside = false;
            }
        }
    }

    pub fn disable_rook(&mut self, color: Color, square: Square) {
        match (color, square.0) {
            (Color::White, 0) => self.white_queenside = false,
            (Color::White, 7) => self.white_kingside = false,
            (Color::Black, 56) => self.black_queenside = false,
            (Color::Black, 63) => self.black_kingside = false,
            _ => {}
        }
    }
}

// ======================================================
// GAME STATE
// ======================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub castling: CastlingRights,

    // Square over which an en passant capture is possible.
    pub en_passant_target: Option<u8>,

    // Number of halfmoves since pawn move or capture.
    pub halfmove_clock: u32,

    // Starts at 1 and increments after Black's move.
    pub fullmove_number: u32,

    // Number of completed plies.
    pub ply: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            castling: CastlingRights::default(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            ply: 0,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_target.and_then(Square::new)
    }

    pub fn update_after_move(
        &mut self,
        color: Color,
        from: Square,
        to: Square,
        is_pawn_move: bool,
        is_capture: bool,
        is_double_pawn_move: bool,
    ) {
        // Reset en passant target.
        self.en_passant_target = None;

        // A double pawn move creates an en passant target.
        if is_double_pawn_move && is_pawn_move {
            let target_rank = match color {
                Color::White => 2,
                Color::Black => 5,
            };

            self.en_passant_target =
                Square::from_coords(from.file(), target_rank)
                    .map(|square| square.0);
        }

        // King movement removes both castling rights.
        if from.0 == 4 && color == Color::White {
            self.castling.disable_king(Color::White);
        }

        if from.0 == 60 && color == Color::Black {
            self.castling.disable_king(Color::Black);
        }

        // Rook movement removes its corresponding right.
        self.castling.disable_rook(color, from);

        // Capturing a rook on its original square removes that right.
        let enemy = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        if is_capture {
            self.castling.disable_rook(enemy, to);
        }

        // Halfmove clock.
        if is_pawn_move || is_capture {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Fullmove number increments after Black.
        if color == Color::Black {
            self.fullmove_number += 1;
        }

        self.ply += 1;
    }

    pub fn is_fifty_move_draw(&self) -> bool {
        self.halfmove_clock >= 100
    }

    pub fn is_seventyfive_move_draw(&self) -> bool {
        self.halfmove_clock >= 150
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn test_initial_castling_rights() {
        let state = GameState::new();

        assert!(state.castling.white_kingside);
        assert!(state.castling.white_queenside);
        assert!(state.castling.black_kingside);
        assert!(state.castling.black_queenside);
    }

    #[test]
    fn test_king_move_disables_castling() {
        let mut state = GameState::new();

        state.update_after_move(
            Color::White,
            Square(4),
            Square(5),
            false,
            false,
            false,
        );

        assert!(!state.castling.white_kingside);
        assert!(!state.castling.white_queenside);
    }

    #[test]
    fn test_rook_move_disables_one_castling_right() {
        let mut state = GameState::new();

        state.update_after_move(
            Color::White,
            Square(0),
            Square(8),
            false,
            false,
            false,
        );

        assert!(!state.castling.white_queenside);
        assert!(state.castling.white_kingside);
    }

    #[test]
    fn test_en_passant_target_after_double_pawn_move() {
        let mut state = GameState::new();

        state.update_after_move(
            Color::White,
            Square(12), // E2
            Square(28), // E4
            true,
            false,
            true,
        );

        assert_eq!(
            state.en_passant_square(),
            Some(Square(20)) // E3
        );
    }

    #[test]
    fn test_pawn_move_resets_halfmove_clock() {
        let mut state = GameState::new();

        state.halfmove_clock = 25;

        state.update_after_move(
            Color::White,
            Square(12),
            Square(20),
            true,
            false,
            false,
        );

        assert_eq!(state.halfmove_clock, 0);
    }

    #[test]
    fn test_capture_resets_halfmove_clock() {
        let mut state = GameState::new();

        state.halfmove_clock = 40;

        state.update_after_move(
            Color::White,
            Square(0),
            Square(8),
            false,
            true,
            false,
        );

        assert_eq!(state.halfmove_clock, 0);
    }

    #[test]
    fn test_black_move_increments_fullmove_number() {
        let mut state = GameState::new();

        state.update_after_move(
            Color::Black,
            Square(48),
            Square(40),
            true,
            false,
            false,
        );

        assert_eq!(state.fullmove_number, 2);
    }

    #[test]
    fn test_fifty_move_draw() {
        let mut state = GameState::new();

        state.halfmove_clock = 100;

        assert!(state.is_fifty_move_draw());
    }

    #[test]
    fn test_seventyfive_move_draw() {
        let mut state = GameState::new();

        state.halfmove_clock = 150;

        assert!(state.is_seventyfive_move_draw());
    }
}
