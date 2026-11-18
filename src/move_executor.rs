
use crate::{
    ChessBoard, Color, Piece,
    game_state::GameState,
    legal_move::LegalMoveValidator,
    move_generator::ChessMove,
    special_moves::SpecialMoves,
    en_passant::EnPassant,
};

pub struct MoveExecutor;

impl MoveExecutor {

    // ==========================================
    // UNIFIED MOVE EXECUTION
    // ==========================================

    pub fn execute(
        board: &ChessBoard,
        state: &GameState,
        chess_move: ChessMove,
    ) -> Option<(ChessBoard, GameState)> {

        let color = board.side_to_move;

        // Source square must contain a piece of the moving side.
        let (piece_color, piece) =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, chess_move.from)?;

        if piece_color != color {
            return None;
        }

        // ======================================
        // 1. CASTLING
        // ======================================

        if piece == Piece::King
            && chess_move.from.file() == 4
            && (chess_move.to.file() == 2
                || chess_move.to.file() == 6)
            && chess_move.from.rank() == chess_move.to.rank()
        {
            return SpecialMoves::try_castle(
                board,
                state,
                chess_move,
            );
        }

        // ======================================
        // 2. EN PASSANT
        // ======================================

        if piece == Piece::Pawn
            && state.en_passant_target == Some(chess_move.to.0)
            && chess_move.from.file() != chess_move.to.file()
        {
            return EnPassant::try_move(
                board,
                state,
                chess_move,
            );
        }

        // ======================================
        // 3. NORMAL MOVE VALIDATION
        // ======================================

        // Promotion must use execute_promotion().
        if piece == Piece::Pawn {
            let final_rank = match color {
                Color::White => 7,
                Color::Black => 0,
            };

            if chess_move.to.rank() == final_rank {
                return None;
            }
        }

        if !LegalMoveValidator::is_legal_move(
            board,
            chess_move,
            color,
        ) {
            return None;
        }

        // Capture information must be read before moving.
        let captured_piece =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, chess_move.to);

        let is_capture = captured_piece.is_some();
        let is_pawn_move = piece == Piece::Pawn;

        // Detect a two-square pawn advance.
        let is_double_pawn_move =
            is_pawn_move
            && match color {
                Color::White =>
                    chess_move.from.rank() == 1
                    && chess_move.to.rank() == 3,

                Color::Black =>
                    chess_move.from.rank() == 6
                    && chess_move.to.rank() == 4,
            };

        // ======================================
        // 4. EXECUTE NORMAL MOVE
        // ======================================

        let next_board = LegalMoveValidator::make_move(
            board,
            chess_move,
        )?;

        // ======================================
        // 5. UPDATE GAME STATE
        // ======================================

        let mut next_state = state.clone();

        next_state.update_after_move(
            color,
            chess_move.from,
            chess_move.to,
            is_pawn_move,
            is_capture,
            is_double_pawn_move,
        );

        Some((next_board, next_state))
    }
}


// ==========================================
// PAWN PROMOTION
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionChoice {
    Queen,
    Rook,
    Bishop,
    Knight,
}

impl PromotionChoice {
    pub fn to_piece(self) -> Piece {
        match self {
            Self::Queen => Piece::Queen,
            Self::Rook => Piece::Rook,
            Self::Bishop => Piece::Bishop,
            Self::Knight => Piece::Knight,
        }
    }
}

impl MoveExecutor {

    pub fn execute_promotion(
        board: &ChessBoard,
        state: &GameState,
        chess_move: ChessMove,
        choice: PromotionChoice,
    ) -> Option<(ChessBoard, GameState)> {

        let color = board.side_to_move;

        // Source must contain the moving side's pawn.
        let (piece_color, piece) =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, chess_move.from)?;

        if piece_color != color || piece != Piece::Pawn {
            return None;
        }

        // Pawn must reach its final rank.
        let final_rank = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        if chess_move.to.rank() != final_rank {
            return None;
        }

        // Validate movement and King safety.
        if !LegalMoveValidator::is_legal_move(
            board,
            chess_move,
            color,
        ) {
            return None;
        }

        let is_capture =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, chess_move.to)
                .is_some();

        // Execute the legal pawn move on a cloned board.
        let mut next_board =
            LegalMoveValidator::make_move(board, chess_move)?;

        // Remove the pawn from the destination.
        next_board.pieces[color.index()][Piece::Pawn as usize]
            .clear(chess_move.to);

        // Place the promoted piece.
        let promoted_piece = choice.to_piece();

        next_board.pieces[color.index()][promoted_piece as usize]
            .set(chess_move.to);

        next_board.rebuild_occupancy();

        // Update game state.
        let mut next_state = state.clone();

        next_state.update_after_move(
            color,
            chess_move.from,
            chess_move.to,
            true,
            is_capture,
            false,
        );

        Some((next_board, next_state))
    }
}
