
use crate::{
    ChessBoard, Color, Piece,
    game_state::GameState,
    legal_move::LegalMoveValidator,
    move_executor::{MoveExecutor, PromotionChoice},
    move_generator::ChessMove,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChessAction {
    Move(ChessMove),
    Promote(ChessMove, PromotionChoice),
}

pub struct ActionExecutor;

impl ActionExecutor {

    // ==========================================
    // GENERATE COMPLETE LEGAL ACTIONS
    // ==========================================

    pub fn generate(
        board: &ChessBoard,
        state: &GameState,
        color: Color,
    ) -> Vec<ChessAction> {

        let moves =
            LegalMoveValidator::generate_complete_legal_moves(
                board,
                state,
                color,
            );

        let mut actions = Vec::new();

        for chess_move in moves {

            let source_is_pawn =
                board.pieces[color.index()][Piece::Pawn as usize]
                    .contains(chess_move.from);

            let final_rank = match color {
                Color::White => 7,
                Color::Black => 0,
            };

            // Promotion move becomes four independent actions.
            if source_is_pawn && chess_move.to.rank() == final_rank {

                let choices = [
                    PromotionChoice::Queen,
                    PromotionChoice::Rook,
                    PromotionChoice::Bishop,
                    PromotionChoice::Knight,
                ];

                for choice in choices {
                    let action =
                        ChessAction::Promote(chess_move, choice);

                    if !actions.contains(&action) {
                        actions.push(action);
                    }
                }

            } else {
                let action = ChessAction::Move(chess_move);

                if !actions.contains(&action) {
                    actions.push(action);
                }
            }
        }

        actions
    }

    // ==========================================
    // EXECUTE ACTION
    // ==========================================

    pub fn execute(
        board: &ChessBoard,
        state: &GameState,
        action: ChessAction,
    ) -> Option<(ChessBoard, GameState)> {

        match action {

            ChessAction::Move(chess_move) => {
                MoveExecutor::execute(
                    board,
                    state,
                    chess_move,
                )
            }

            ChessAction::Promote(chess_move, choice) => {
                MoveExecutor::execute_promotion(
                    board,
                    state,
                    chess_move,
                    choice,
                )
            }
        }
    }

    // ==========================================
    // CHECKMATE WITH COMPLETE RULES
    // ==========================================

    pub fn is_checkmate(
        board: &ChessBoard,
        state: &GameState,
        color: Color,
    ) -> bool {

        LegalMoveValidator::is_in_check(board, color)
            && Self::generate(board, state, color).is_empty()
    }

    // ==========================================
    // STALEMATE WITH COMPLETE RULES
    // ==========================================

    pub fn is_stalemate(
        board: &ChessBoard,
        state: &GameState,
        color: Color,
    ) -> bool {

        !LegalMoveValidator::is_in_check(board, color)
            && Self::generate(board, state, color).is_empty()
    }
}
