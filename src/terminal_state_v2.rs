
use crate::{
    ChessBoard,
    Color,
    Piece,
    game_state::GameState,
    chess_action::ActionExecutor,
    en_passant::EnPassant,
    terminal_state::{
        TerminalState,
        TerminalEvaluator,
        RepetitionTracker,
        GameResult,
    },
};

pub struct TerminalEvaluatorV2;

impl TerminalEvaluatorV2 {
    pub fn evaluate(
        board: &ChessBoard,
        state: &GameState,
        repetition_count: u32,
    ) -> TerminalState {
        TerminalEvaluator::evaluate(
            board,
            state,
            repetition_count,
        )
    }

    pub fn is_checkmate(
        board: &ChessBoard,
        state: &GameState,
    ) -> bool {
        ActionExecutor::is_checkmate(
            board,
            state,
            board.side_to_move,
        )
    }

    pub fn is_stalemate(
        board: &ChessBoard,
        state: &GameState,
    ) -> bool {
        ActionExecutor::is_stalemate(
            board,
            state,
            board.side_to_move,
        )
    }

    pub fn is_insufficient_material(
        board: &ChessBoard,
    ) -> bool {
        TerminalEvaluator::is_insufficient_material(board)
    }

    pub fn result(state: TerminalState) -> GameResult {
        TerminalEvaluator::result(state)
    }
}

pub struct RepetitionTrackerV2;

impl RepetitionTrackerV2 {
    // FIDE repetition identity:
    // Piece placement + side to move + castling rights
    // + legally available en-passant capture.

    pub fn position_key(
        board: &ChessBoard,
        state: &GameState,
    ) -> String {
        let mut key = String::new();

        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                let index = color.index();
                let piece_index = piece as usize;

                key.push_str(&format!(
                    "{}:{};",
                    index,
                    board.pieces[index][piece_index].0
                ));
            }
        }

        key.push_str(&format!(
            "turn:{:?};",
            board.side_to_move
        ));

        key.push_str(&format!(
            "castle:{}{}{}{};",
            state.castling.white_kingside as u8,
            state.castling.white_queenside as u8,
            state.castling.black_kingside as u8,
            state.castling.black_queenside as u8,
        ));

        // EP target matters only if a legal EP capture exists.
        let legal_ep =
            EnPassant::generate_legal_moves(board, state);

        if legal_ep.is_empty() {
            key.push_str("ep:none");
        } else {
            key.push_str(&format!(
                "ep:{:?}",
                state.en_passant_target
            ));
        }

        key
    }
}

pub struct RepetitionTrackerV2State {
    tracker: RepetitionTracker,
}

impl RepetitionTrackerV2State {
    pub fn new() -> Self {
        Self {
            tracker: RepetitionTracker::new(),
        }
    }

    pub fn record(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
    ) -> u32 {
        let key =
            RepetitionTrackerV2::position_key(board, state);

        self.tracker.record_key(key)
    }

    pub fn count(
        &self,
        board: &ChessBoard,
        state: &GameState,
    ) -> u32 {
        let key =
            RepetitionTrackerV2::position_key(board, state);

        self.tracker.count_key(&key)
    }
}
