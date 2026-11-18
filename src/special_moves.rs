use crate::{
    game_state::GameState,
    move_generator::{ChessMove, MoveGenerator},
    ChessBoard, Color, Piece, Square,
};

pub struct SpecialMoves;

impl SpecialMoves {

    // ==================================================
    // CASTLING
    // ==================================================

    pub fn try_castle(
        board: &ChessBoard,
        state: &GameState,
        chess_move: ChessMove,
    ) -> Option<(ChessBoard, GameState)> {

        let color = board.side_to_move;

        let (king_from, king_side_to, queen_side_to,
             rook_king_from, rook_queen_from,
             rook_king_to, rook_queen_to) = match color {

            Color::White => (
                4, 6, 2,
                7, 0,
                5, 3,
            ),

            Color::Black => (
                60, 62, 58,
                63, 56,
                61, 59,
            ),
        };

        // Must start from the correct King square.
        if chess_move.from != Square(king_from) {
            return None;
        }

        let is_kingside = chess_move.to.0 == king_side_to;
        let is_queenside = chess_move.to.0 == queen_side_to;

        if !is_kingside && !is_queenside {
            return None;
        }

        // Verify castling rights.
        let rights = match color {
            Color::White => (
                state.castling.white_kingside,
                state.castling.white_queenside,
            ),
            Color::Black => (
                state.castling.black_kingside,
                state.castling.black_queenside,
            ),
        };

        if is_kingside && !rights.0 {
            return None;
        }

        if is_queenside && !rights.1 {
            return None;
        }

        // Verify King and Rook exist.
        if !board.pieces[color.index()][Piece::King as usize]
            .contains(Square(king_from))
        {
            return None;
        }

        let rook_from = if is_kingside {
            rook_king_from
        } else {
            rook_queen_from
        };

        let rook_to = if is_kingside {
            rook_king_to
        } else {
            rook_queen_to
        };

        if !board.pieces[color.index()][Piece::Rook as usize]
            .contains(Square(rook_from))
        {
            return None;
        }

        // Verify path is empty.
        let empty_squares: &[u8] = if is_kingside {
            if color == Color::White {
                &[5, 6]
            } else {
                &[61, 62]
            }
        } else {
            if color == Color::White {
                &[1, 2, 3]
            } else {
                &[57, 58, 59]
            }
        };

        for &index in empty_squares {
            if board.all_occupied.contains(Square(index)) {
                return None;
            }
        }

        // King cannot castle out of, through, or into check.
        let enemy = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        let king_path: &[u8] = if is_kingside {
            &[king_from, king_from + 1, king_from + 2]
        } else {
            &[king_from, king_from - 1, king_from - 2]
        };

        for &index in king_path {
            if MoveGenerator::is_square_attacked(
                board,
                Square(index),
                enemy,
            ) {
                return None;
            }
        }

        // Create new board.
        let mut next_board = board.clone();

        // Move King.
        next_board.pieces[color.index()][Piece::King as usize]
            .clear(Square(king_from));

        next_board.pieces[color.index()][Piece::King as usize]
            .set(chess_move.to);

        // Move Rook.
        next_board.pieces[color.index()][Piece::Rook as usize]
            .clear(Square(rook_from));

        next_board.pieces[color.index()][Piece::Rook as usize]
            .set(Square(rook_to));

        next_board.rebuild_occupancy();

        // Update GameState.
        let mut next_state = state.clone();

        next_state.update_after_move(
            color,
            Square(king_from),
            chess_move.to,
            false,
            false,
            false,
        );

        // Update turn.
        next_board.side_to_move = enemy;

        Some((next_board, next_state))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameState;

    fn castle_board() -> ChessBoard {
        let mut board = ChessBoard::empty();

        board.add_piece(Color::White, Piece::King, Square(4));
        board.add_piece(Color::White, Piece::Rook, Square(0));
        board.add_piece(Color::White, Piece::Rook, Square(7));

        board.add_piece(Color::Black, Piece::King, Square(60));

        board
    }

    #[test]
    fn test_white_kingside_castling() {
        let board = castle_board();
        let state = GameState::new();

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(6),
        };

        let (next, next_state) =
            SpecialMoves::try_castle(&board, &state, chess_move)
                .unwrap();

        assert!(next.pieces[0][Piece::King as usize].contains(Square(6)));
        assert!(next.pieces[0][Piece::Rook as usize].contains(Square(5)));
        assert_eq!(next.side_to_move, Color::Black);
        assert!(!next_state.castling.white_kingside);
        assert!(!next_state.castling.white_queenside);
    }

    #[test]
    fn test_white_queenside_castling() {
        let board = castle_board();
        let state = GameState::new();

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(2),
        };

        let (next, _) =
            SpecialMoves::try_castle(&board, &state, chess_move)
                .unwrap();

        assert!(next.pieces[0][Piece::King as usize].contains(Square(2)));
        assert!(next.pieces[0][Piece::Rook as usize].contains(Square(3)));
    }

    #[test]
    fn test_castling_rejected_when_path_blocked() {
        let mut board = castle_board();
        board.add_piece(Color::White, Piece::Knight, Square(5));

        let state = GameState::new();

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(6),
        };

        assert!(
            SpecialMoves::try_castle(&board, &state, chess_move).is_none()
        );
    }

    #[test]
    fn test_castling_rejected_when_king_in_check() {
        let mut board = castle_board();

        board.add_piece(Color::Black, Piece::Rook, Square(60));
        board.pieces[1][Piece::King as usize].clear(Square(60));
        board.add_piece(Color::Black, Piece::King, Square(56));

        let state = GameState::new();

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(6),
        };

        assert!(
            SpecialMoves::try_castle(&board, &state, chess_move).is_none()
        );
    }

    #[test]
    fn test_castling_rejected_without_rights() {
        let board = castle_board();
        let mut state = GameState::new();

        state.castling.white_kingside = false;

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(6),
        };

        assert!(
            SpecialMoves::try_castle(&board, &state, chess_move).is_none()
        );
    }

    #[test]
    fn test_original_board_unchanged() {
        let board = castle_board();
        let before = board.clone();
        let state = GameState::new();

        let chess_move = ChessMove {
            from: Square(4),
            to: Square(6),
        };

        let _ = SpecialMoves::try_castle(&board, &state, chess_move);

        assert_eq!(board.all_occupied.0, before.all_occupied.0);
        assert_eq!(board.side_to_move, before.side_to_move);
    }
}
