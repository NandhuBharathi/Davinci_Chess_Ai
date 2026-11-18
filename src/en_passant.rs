
use crate::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::{ChessMove, MoveGenerator},
};

pub struct EnPassant;

impl EnPassant {

    // -----------------------------------------
    // FIND KING
    // -----------------------------------------

    fn find_king(board: &ChessBoard, color: Color) -> Option<Square> {
        let mut pieces =
            board.pieces[color.index()][Piece::King as usize];

        pieces.pop_lsb()
    }

    // -----------------------------------------
    // CHECK EN PASSANT GEOMETRY
    // -----------------------------------------

    pub fn is_en_passant(
        board: &ChessBoard,
        state: &GameState,
        mv: ChessMove,
    ) -> bool {

        let color = board.side_to_move;

        if state.en_passant_target != Some(mv.to.0) {
            return false;
        }

        let from_rank = mv.from.rank();
        let to_rank = mv.to.rank();

        let file_difference =
            (mv.from.file() as i8 - mv.to.file() as i8).abs();

        if file_difference != 1 {
            return false;
        }

        match color {
            Color::White => {
                from_rank == 4 && to_rank == 5
            }
            Color::Black => {
                from_rank == 3 && to_rank == 2
            }
        }
    }

    // -----------------------------------------
    // EXECUTE EN PASSANT
    // -----------------------------------------

    pub fn try_move(
        board: &ChessBoard,
        state: &GameState,
        mv: ChessMove,
    ) -> Option<(ChessBoard, GameState)> {

        let color = board.side_to_move;
        let enemy = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        if !Self::is_en_passant(board, state, mv) {
            return None;
        }

        // Source must contain own pawn.
        if !board.pieces[color.index()][Piece::Pawn as usize]
            .contains(mv.from)
        {
            return None;
        }

        // Target must be empty.
        if board.all_occupied.contains(mv.to) {
            return None;
        }

        // Captured pawn is behind the target square.
        let captured_index = match color {
            Color::White => mv.to.0.checked_sub(8)?,
            Color::Black => mv.to.0.checked_add(8)?,
        };

        let captured_square = Square::new(captured_index)?;

        if !board.pieces[enemy.index()][Piece::Pawn as usize]
            .contains(captured_square)
        {
            return None;
        }

        // Clone board.
        let mut next_board = board.clone();

        // Move attacking pawn.
        next_board.pieces[color.index()][Piece::Pawn as usize]
            .clear(mv.from);

        next_board.pieces[color.index()][Piece::Pawn as usize]
            .set(mv.to);

        // Remove captured pawn.
        next_board.pieces[enemy.index()][Piece::Pawn as usize]
            .clear(captured_square);

        next_board.rebuild_occupancy();

        // Own king must remain safe.
        let king_square = Self::find_king(&next_board, color)?;

        if MoveGenerator::is_square_attacked(
            &next_board,
            king_square,
            enemy,
        ) {
            return None;
        }

        // Update game state.
        let mut next_state = state.clone();

        next_state.update_after_move(
            color,
            mv.from,
            mv.to,
            true,
            true,
            false,
        );

        // Change turn.
        next_board.side_to_move = enemy;

        Some((next_board, next_state))
    }

    // -----------------------------------------
    // GENERATE LEGAL EN PASSANT MOVES
    // -----------------------------------------

    pub fn generate_legal_moves(
        board: &ChessBoard,
        state: &GameState,
    ) -> Vec<ChessMove> {

        let color = board.side_to_move;
        let mut moves = Vec::new();

        let target = match state.en_passant_target {
            Some(value) => Square(value),
            None => return moves,
        };

        let mut pawns =
            board.pieces[color.index()][Piece::Pawn as usize];

        while let Some(from) = pawns.pop_lsb() {

            let file_difference =
                (from.file() as i8 - target.file() as i8).abs();

            if file_difference != 1 {
                continue;
            }

            let mv = ChessMove {
                from,
                to: target,
            };

            if Self::try_move(board, state, mv).is_some() {
                moves.push(mv);
            }
        }

        moves
    }
}
