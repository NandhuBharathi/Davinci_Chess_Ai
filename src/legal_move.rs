use crate::{
    move_generator::{ChessMove, MoveGenerator},
    ChessBoard, Color, Piece, Square,
};

pub struct LegalMoveValidator;

impl LegalMoveValidator {

    // --------------------------------------------------
    // 1. FIND KING
    // --------------------------------------------------

    pub fn find_king(
        board: &ChessBoard,
        color: Color,
    ) -> Option<Square> {

        let king_board =
            board.pieces[color.index()][Piece::King as usize];

        let mut bits = king_board;

        bits.pop_lsb()
    }

    // --------------------------------------------------
    // 2. CHECK DETECTION
    // --------------------------------------------------

    pub fn is_in_check(
        board: &ChessBoard,
        color: Color,
    ) -> bool {

        let king_square = match Self::find_king(board, color) {
            Some(square) => square,
            None => return true,
        };

        let enemy = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        MoveGenerator::is_square_attacked(
            board,
            king_square,
            enemy,
        )
    }

    // --------------------------------------------------
    // 3. MAKE MOVE ON TEMPORARY BOARD
    // --------------------------------------------------

    pub fn make_move(
        board: &ChessBoard,
        chess_move: ChessMove,
    ) -> Option<ChessBoard> {

        let (color, piece) =
            MoveGenerator::piece_at_for_validator(
                board,
                chess_move.from,
            )?;

        // Only the player whose turn it is can move.
        if color != board.side_to_move {
            return None;
        }

        // Cannot capture own piece.
        if MoveGenerator::color_at_for_validator(
            board,
            chess_move.to,
        ) == Some(color) {
            return None;
        }

        // King capture is never a legal chess move.
        if MoveGenerator::piece_at_for_validator(
            board,
            chess_move.to,
        ).is_some_and(|(_, p)| p == Piece::King) {
            return None;
        }

        let mut next_board = board.clone();

        next_board.pieces[color.index()][piece as usize]
            .clear(chess_move.from);

        if let Some((target_color, target_piece)) =
            MoveGenerator::piece_at_for_validator(
                board,
                chess_move.to,
            )
        {
            next_board.pieces[target_color.index()]
                [target_piece as usize]
                .clear(chess_move.to);
        }

        next_board.pieces[color.index()][piece as usize]
            .set(chess_move.to);

        next_board.side_to_move = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        next_board.rebuild_occupancy();

        Some(next_board)
    }

    // --------------------------------------------------
    // 4. LEGAL MOVE CHECK
    // --------------------------------------------------

    pub fn is_legal_move(
        board: &ChessBoard,
        chess_move: ChessMove,
        color: Color,
    ) -> bool {

        // Reject moves when it is not this player's turn.
        if board.side_to_move != color {
            return false;
        }

        if MoveGenerator::color_at_for_validator(
            board,
            chess_move.from,
        ) != Some(color) {
            return false;
        }

        let pseudo_moves =
            MoveGenerator::generate_pseudo_legal(
                board,
                chess_move.from,
            );

        if !pseudo_moves.contains(&chess_move) {
            return false;
        }

        let next_board = match Self::make_move(
            board,
            chess_move,
        ) {
            Some(b) => b,
            None => return false,
        };

        !Self::is_in_check(&next_board, color)
    }

    // --------------------------------------------------
    // 5. GENERATE LEGAL MOVES
    // --------------------------------------------------

    pub fn generate_legal_moves(
        board: &ChessBoard,
        color: Color,
    ) -> Vec<ChessMove> {

        let mut legal_moves = Vec::new();

        for index in 0..64 {
            let from = Square(index);

            if MoveGenerator::color_at_for_validator(
                board,
                from,
            ) != Some(color) {
                continue;
            }

            let candidates =
                MoveGenerator::generate_pseudo_legal(
                    board,
                    from,
                );

            for chess_move in candidates {
                if Self::is_legal_move(
                    board,
                    chess_move,
                    color,
                ) {
                    legal_moves.push(chess_move);
                }
            }
        }

        legal_moves
    }

    // --------------------------------------------------
    // 6. CHECKMATE
    // --------------------------------------------------

    pub fn is_checkmate(
        board: &ChessBoard,
        color: Color,
    ) -> bool {

        Self::is_in_check(board, color)
            && Self::generate_legal_moves(board, color).is_empty()
    }

    // --------------------------------------------------
    // 7. STALEMATE
    // --------------------------------------------------

    pub fn is_stalemate(
        board: &ChessBoard,
        color: Color,
    ) -> bool {

        !Self::is_in_check(board, color)
            && Self::generate_legal_moves(board, color).is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::move_generator::ChessMove;

    fn empty_board() -> ChessBoard {
        let mut board = ChessBoard::empty();

        board.add_piece(
            Color::White,
            Piece::King,
            Square::new(4).unwrap(),
        );

        board.add_piece(
            Color::Black,
            Piece::King,
            Square::new(60).unwrap(),
        );

        board
    }

    #[test]
    fn test_king_check_detection() {
        let mut board = empty_board();

        board.add_piece(
            Color::Black,
            Piece::Rook,
            Square::new(12).unwrap(), // E2
        );

        assert!(
            LegalMoveValidator::is_in_check(
                &board,
                Color::White
            )
        );
    }

    #[test]
    fn test_king_cannot_move_into_check() {
        let mut board = empty_board();

        board.add_piece(
            Color::Black,
            Piece::Rook,
            Square::new(13).unwrap(), // F2
        );

        let chess_move = ChessMove {
            from: Square::new(4).unwrap(),  // E1
            to: Square::new(5).unwrap(),    // F1
        };

        assert!(
            !LegalMoveValidator::is_legal_move(
                &board,
                chess_move,
                Color::White
            )
        );
    }

    #[test]
    fn test_pinned_piece_cannot_expose_king() {
        let mut board = ChessBoard::empty();

        board.add_piece(
            Color::White,
            Piece::King,
            Square::new(4).unwrap(), // E1
        );

        board.add_piece(
            Color::White,
            Piece::Rook,
            Square::new(12).unwrap(), // E2
        );

        board.add_piece(
            Color::Black,
            Piece::Rook,
            Square::new(60).unwrap(), // E8
        );

        board.add_piece(
            Color::Black,
            Piece::King,
            Square::new(56).unwrap(), // A8
        );

        let chess_move = ChessMove {
            from: Square::new(12).unwrap(),
            to: Square::new(13).unwrap(),
        };

        assert!(
            !LegalMoveValidator::is_legal_move(
                &board,
                chess_move,
                Color::White
            )
        );
    }

    #[test]
    fn test_checkmate_detection() {
        let mut board = ChessBoard::empty();

        board.add_piece(
            Color::White,
            Piece::King,
            Square::new(0).unwrap(), // A1
        );

        board.add_piece(
            Color::Black,
            Piece::King,
            Square::new(18).unwrap(), // C3
        );

        board.add_piece(
            Color::Black,
            Piece::Queen,
            Square::new(9).unwrap(), // B2
        );

        assert!(
            LegalMoveValidator::is_checkmate(
                &board,
                Color::White
            )
        );
    }

    #[test]
    fn test_stalemate_detection() {
        let mut board = ChessBoard::empty();

        board.add_piece(
            Color::White,
            Piece::King,
            Square::new(0).unwrap(), // A1
        );

        board.add_piece(
            Color::Black,
            Piece::King,
            Square::new(18).unwrap(), // C3
        );

        board.add_piece(
            Color::Black,
            Piece::Queen,
            Square::new(10).unwrap(), // C2
        );

        assert!(
            LegalMoveValidator::is_stalemate(
                &board,
                Color::White
            )
        );
    }

    #[test]
    fn test_original_board_unchanged() {
        let board = ChessBoard::initial_position();

        let before = board.clone();

        let chess_move = ChessMove {
            from: Square::new(12).unwrap(), // E2
            to: Square::new(28).unwrap(),   // E4
        };

        let _ = LegalMoveValidator::make_move(
            &board,
            chess_move
        );

        assert_eq!(
            board.all_occupied.0,
            before.all_occupied.0
        );

        assert_eq!(
            board.occupied[0].0,
            before.occupied[0].0
        );

        assert_eq!(
            board.occupied[1].0,
            before.occupied[1].0
        );
    }
}


#[cfg(test)]
mod turn_tests {
    use crate::{ChessBoard, Color, Square};
    use crate::legal_move::LegalMoveValidator;
    use crate::move_generator::ChessMove;

    #[test]
    fn test_white_moves_first() {
        let board = ChessBoard::initial_position();

        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_black_cannot_move_first() {
        let board = ChessBoard::initial_position();

        let black_move = ChessMove {
            from: Square(52), // E7
            to: Square(36),   // E5
        };

        assert!(
            !LegalMoveValidator::is_legal_move(
                &board,
                black_move,
                Color::Black
            )
        );
    }

    #[test]
    fn test_white_move_changes_turn_to_black() {
        let board = ChessBoard::initial_position();

        let chess_move = ChessMove {
            from: Square(12), // E2
            to: Square(28),   // E4
        };

        let next = LegalMoveValidator::make_move(
            &board,
            chess_move
        ).unwrap();

        assert_eq!(next.side_to_move, Color::Black);
    }

    #[test]
    fn test_black_move_changes_turn_to_white() {
        let mut board = ChessBoard::initial_position();

        board.side_to_move = Color::Black;

        let chess_move = ChessMove {
            from: Square(52), // E7
            to: Square(36),   // E5
        };

        let next = LegalMoveValidator::make_move(
            &board,
            chess_move
        ).unwrap();

        assert_eq!(next.side_to_move, Color::White);
    }

    #[test]
    fn test_wrong_turn_does_not_change_board() {
        let board = ChessBoard::initial_position();
        let before = board.clone();

        let chess_move = ChessMove {
            from: Square(52),
            to: Square(36),
        };

        assert!(
            LegalMoveValidator::make_move(
                &board,
                chess_move
            ).is_none()
        );

        assert_eq!(board.all_occupied.0, before.all_occupied.0);
        assert_eq!(board.side_to_move, before.side_to_move);
    }
}


// ==========================================
// LEGAL MOVES + CASTLING INTEGRATION
// ==========================================

impl LegalMoveValidator {
    pub fn generate_legal_moves_with_state(
        board: &ChessBoard,
        state: &crate::game_state::GameState,
        color: Color,
    ) -> Vec<crate::move_generator::ChessMove> {

        let mut moves = Self::generate_legal_moves(board, color);

        // Castling is possible only for the side whose turn it is.
        if board.side_to_move != color {
            return moves;
        }

        let candidates = match color {
            Color::White => [
                (4u8, 6u8),  // E1 -> G1
                (4u8, 2u8),  // E1 -> C1
            ],
            Color::Black => [
                (60u8, 62u8), // E8 -> G8
                (60u8, 58u8), // E8 -> C8
            ],
        };

        for (from, to) in candidates {
            let chess_move = crate::move_generator::ChessMove {
                from: Square(from),
                to: Square(to),
            };

            if crate::special_moves::SpecialMoves::try_castle(
                board,
                state,
                chess_move,
            ).is_some() {
                moves.push(chess_move);
            }
        }

        moves
    }
}


// ==========================================
// COMPLETE LEGAL MOVE GENERATOR
// Normal Moves + Castling + En Passant
// ==========================================

impl LegalMoveValidator {

    pub fn generate_complete_legal_moves(
        board: &ChessBoard,
        state: &crate::game_state::GameState,
        color: Color,
    ) -> Vec<crate::move_generator::ChessMove> {

        let mut moves =
            Self::generate_legal_moves_with_state(board, state, color);

        if board.side_to_move != color {
            return moves;
        }

        let en_passant_moves =
            crate::en_passant::EnPassant::generate_legal_moves(
                board,
                state,
            );

        moves.extend(en_passant_moves);

        moves
    }
}
