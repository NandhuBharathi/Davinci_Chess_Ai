
use std::collections::HashMap;

use crate::{
    ChessBoard, Color, Piece,
    game_state::GameState,
    chess_action::ActionExecutor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState {
    Ongoing,
    Checkmate { winner: Color },
    Stalemate,
    FiftyMoveClaimable,
    SeventyFiveMoveDraw,
    ThreefoldRepetitionClaimable,
    FivefoldRepetitionDraw,
    InsufficientMaterial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
    Ongoing,
}

pub struct RepetitionTracker {
    positions: HashMap<String, u32>,
}

impl RepetitionTracker {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

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

        key.push_str(&format!(
            "ep:{:?}",
            state.en_passant_target
        ));

        key
    }

    pub fn record(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
    ) -> u32 {
        let key = Self::position_key(board, state);
        self.record_key(key)
    }

    pub fn count(
        &self,
        board: &ChessBoard,
        state: &GameState,
    ) -> u32 {
        let key = Self::position_key(board, state);
        self.count_key(&key)
    }

    pub fn record_key(&mut self, key: String) -> u32 {
        let count = self.positions.entry(key).or_insert(0);
        *count += 1;
        *count
    }

    pub fn count_key(&self, key: &str) -> u32 {
        *self.positions.get(key).unwrap_or(&0)
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }
}

impl Default for RepetitionTracker {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TerminalEvaluator;

impl TerminalEvaluator {
    pub fn evaluate(
        board: &ChessBoard,
        state: &GameState,
        repetition_count: u32,
    ) -> TerminalState {
        let side = board.side_to_move;

        if ActionExecutor::is_checkmate(board, state, side) {
            let winner = match side {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };

            return TerminalState::Checkmate { winner };
        }

        if ActionExecutor::is_stalemate(board, state, side) {
            return TerminalState::Stalemate;
        }

        if state.halfmove_clock >= 150 {
            return TerminalState::SeventyFiveMoveDraw;
        }

        if repetition_count >= 5 {
            return TerminalState::FivefoldRepetitionDraw;
        }

        if Self::is_insufficient_material(board) {
            return TerminalState::InsufficientMaterial;
        }

        if state.halfmove_clock >= 100 {
            return TerminalState::FiftyMoveClaimable;
        }

        if repetition_count >= 3 {
            return TerminalState::ThreefoldRepetitionClaimable;
        }

        TerminalState::Ongoing
    }

    pub fn is_insufficient_material(board: &ChessBoard) -> bool {
        let white_pawns = board.piece_count(Color::White, Piece::Pawn);
        let black_pawns = board.piece_count(Color::Black, Piece::Pawn);

        let white_queens = board.piece_count(Color::White, Piece::Queen);
        let black_queens = board.piece_count(Color::Black, Piece::Queen);

        let white_rooks = board.piece_count(Color::White, Piece::Rook);
        let black_rooks = board.piece_count(Color::Black, Piece::Rook);

        if white_pawns + black_pawns > 0
            || white_queens + black_queens > 0
            || white_rooks + black_rooks > 0
        {
            return false;
        }

        let white_knights = board.piece_count(Color::White, Piece::Knight);
        let black_knights = board.piece_count(Color::Black, Piece::Knight);

        let white_bishops = board.piece_count(Color::White, Piece::Bishop);
        let black_bishops = board.piece_count(Color::Black, Piece::Bishop);

        let knights = white_knights + black_knights;
        let bishops = white_bishops + black_bishops;
        let minors = knights + bishops;

        if minors == 0 || minors == 1 {
            return true;
        }

        if knights == 0 {
            let mut square_colors = Vec::new();

            for color in [Color::White, Color::Black] {
                let bishops_bb =
                    board.pieces[color.index()][Piece::Bishop as usize];

                let mut bits = bishops_bb.0;

                while bits != 0 {
                    let index = bits.trailing_zeros() as u8;
                    bits &= bits - 1;

                    let file = index % 8;
                    let rank = index / 8;

                    square_colors.push((file + rank) % 2);
                }
            }

            return square_colors
                .first()
                .map(|first| square_colors.iter().all(|c| c == first))
                .unwrap_or(true);
        }

        false
    }

    pub fn result(state: TerminalState) -> GameResult {
        match state {
            TerminalState::Checkmate {
                winner: Color::White,
            } => GameResult::WhiteWin,

            TerminalState::Checkmate {
                winner: Color::Black,
            } => GameResult::BlackWin,

            TerminalState::Stalemate
            | TerminalState::SeventyFiveMoveDraw
            | TerminalState::FivefoldRepetitionDraw
            | TerminalState::InsufficientMaterial => GameResult::Draw,

            TerminalState::Ongoing
            | TerminalState::FiftyMoveClaimable
            | TerminalState::ThreefoldRepetitionClaimable => {
                GameResult::Ongoing
            }
        }
    }
}
