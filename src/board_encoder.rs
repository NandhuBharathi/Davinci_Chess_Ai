
use crate::{ChessBoard, Color};
use crate::game_state::GameState;

pub const INPUT_PLANES: usize = 20;
pub const BOARD_SIZE: usize = 64;
pub const ENCODED_SIZE: usize = INPUT_PLANES * BOARD_SIZE;

pub struct BoardEncoder;

impl BoardEncoder {

    // Backward-compatible default-state encoder.
    pub fn encode(board: &ChessBoard) -> Vec<f32> {
        Self::encode_with_state(board, &GameState::new())
    }

    pub fn encode_with_state(
        board: &ChessBoard,
        state: &GameState,
    ) -> Vec<f32> {

        let mut output = vec![0.0f32; ENCODED_SIZE];

        // --------------------------------------
        // 0-11: Piece planes
        // --------------------------------------

        for color_index in 0..2 {
            for piece_index in 0..6 {

                let plane = color_index * 6 + piece_index;

                let bitboard =
                    board.pieces[color_index][piece_index].0;

                for square in 0..64 {
                    let occupied = (bitboard >> square) & 1;

                    output[plane * 64 + square] =
                        occupied as f32;
                }
            }
        }

        // --------------------------------------
        // 12: Side to move
        // --------------------------------------

        let side_value = match board.side_to_move {
            Color::White => 1.0,
            Color::Black => 0.0,
        };

        Self::fill_plane(&mut output, 12, side_value);

        // --------------------------------------
        // 13-16: Castling rights
        // --------------------------------------

        let castling = [
            state.castling.white_kingside,
            state.castling.white_queenside,
            state.castling.black_kingside,
            state.castling.black_queenside,
        ];

        for (index, &right) in castling.iter().enumerate() {
            Self::fill_plane(
                &mut output,
                13 + index,
                if right { 1.0 } else { 0.0 },
            );
        }

        // --------------------------------------
        // 17: En passant target
        // --------------------------------------

        if let Some(square) = state.en_passant_target {
            let square = square as usize;

            if square < 64 {
                output[17 * 64 + square] = 1.0;
            }
        }

        // --------------------------------------
        // 18: Halfmove clock
        // Normalized to 0..1
        // --------------------------------------

        let halfmove =
            state.halfmove_clock.min(100) as f32 / 100.0;

        Self::fill_plane(&mut output, 18, halfmove);

        // --------------------------------------
        // 19: Fullmove number
        // Normalized to 0..1
        // --------------------------------------

        let fullmove =
            state.fullmove_number.min(200) as f32 / 200.0;

        Self::fill_plane(&mut output, 19, fullmove);

        output
    }

    fn fill_plane(
        output: &mut [f32],
        plane: usize,
        value: f32,
    ) {
        let start = plane * 64;
        let end = start + 64;

        output[start..end].fill(value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ChessBoard;

    #[test]
    fn encoder_has_1280_features() {
        let board = ChessBoard::initial_position();
        let state = GameState::new();

        let encoded =
            BoardEncoder::encode_with_state(&board, &state);

        assert_eq!(encoded.len(), 1280);
    }

    #[test]
    fn initial_position_has_32_pieces() {
        let board = ChessBoard::initial_position();
        let encoded = BoardEncoder::encode(&board);

        let piece_features: f32 =
            encoded[..768].iter().sum();

        assert_eq!(piece_features, 32.0);
    }

    #[test]
    fn white_to_move_plane_is_one() {
        let board = ChessBoard::initial_position();
        let encoded = BoardEncoder::encode(&board);

        for value in &encoded[768..832] {
            assert_eq!(*value, 1.0);
        }
    }

    #[test]
    fn initial_castling_rights_are_encoded() {
        let board = ChessBoard::initial_position();
        let state = GameState::new();

        let encoded =
            BoardEncoder::encode_with_state(&board, &state);

        for plane in 13..17 {
            for square in 0..64 {
                assert_eq!(
                    encoded[plane * 64 + square],
                    1.0
                );
            }
        }
    }

    #[test]
    fn en_passant_target_is_encoded() {
        let board = ChessBoard::initial_position();

        let mut state = GameState::new();
        state.en_passant_target = Some(20);

        let encoded =
            BoardEncoder::encode_with_state(&board, &state);

        assert_eq!(encoded[17 * 64 + 20], 1.0);

        let total: f32 =
            encoded[17 * 64..18 * 64].iter().sum();

        assert_eq!(total, 1.0);
    }

    #[test]
    fn halfmove_clock_is_normalized() {
        let board = ChessBoard::initial_position();

        let mut state = GameState::new();
        state.halfmove_clock = 50;

        let encoded =
            BoardEncoder::encode_with_state(&board, &state);

        for value in &encoded[18 * 64..19 * 64] {
            assert_eq!(*value, 0.5);
        }
    }

    #[test]
    fn fullmove_number_is_normalized() {
        let board = ChessBoard::initial_position();

        let mut state = GameState::new();
        state.fullmove_number = 100;

        let encoded =
            BoardEncoder::encode_with_state(&board, &state);

        for value in &encoded[19 * 64..20 * 64] {
            assert_eq!(*value, 0.5);
        }
    }
}
