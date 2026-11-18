
use crate::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::MoveGenerator,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct KingSafetyBreakdown {
    pub white_score: f32,
    pub black_score: f32,
    pub total: f32,
}

pub struct KingSafety;

impl KingSafety {
    const CHECK_PENALTY: f32 = 2.0;
    const DEFENDER_BONUS: f32 = 0.15;
    const PAWN_SHIELD_BONUS: f32 = 0.30;
    const ESCAPE_BONUS: f32 = 0.10;
    const ENEMY_ATTACK_PENALTY: f32 = 0.40;
    const CASTLING_RIGHTS_BONUS: f32 = 0.20;

    pub fn evaluate(
        board: &ChessBoard,
        state: &GameState,
    ) -> KingSafetyBreakdown {
        let white_score = Self::evaluate_side(
            board, state, Color::White
        );

        let black_score = Self::evaluate_side(
            board, state, Color::Black
        );

        KingSafetyBreakdown {
            white_score,
            black_score,
            total: white_score - black_score,
        }
    }

    fn evaluate_side(
        board: &ChessBoard,
        state: &GameState,
        color: Color,
    ) -> f32 {
        let Some(king) = Self::find_king(board, color) else {
            return -100.0;
        };

        let enemy = Self::opponent(color);
        let mut score = 0.0;

        // 1. Check detection
        if MoveGenerator::is_square_attacked(board, king, enemy) {
            score -= Self::CHECK_PENALTY;
        }

        // 2. Nearby friendly defenders
        score += Self::friendly_defenders(
            board, king, color
        ) as f32 * Self::DEFENDER_BONUS;

        // 3. Pawn shield
        score += Self::pawn_shield(
            board, king, color
        ) as f32 * Self::PAWN_SHIELD_BONUS;

        // 4. Safe escape squares
        score += Self::escape_squares(
            board, king, color
        ) as f32 * Self::ESCAPE_BONUS;

        // 5. Enemy attacks around the King
        score -= Self::enemy_pressure(
            board, king, enemy
        ) as f32 * Self::ENEMY_ATTACK_PENALTY;

        // 6. Castling rights
        if Self::has_castling_rights(state, color) {
            score += Self::CASTLING_RIGHTS_BONUS;
        }

        score
    }

    fn find_king(
        board: &ChessBoard,
        color: Color,
    ) -> Option<Square> {
        let king_index = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::King)?;

        let mut kings = board.pieces[color.index()][king_index];

        kings.pop_lsb()
    }

    fn friendly_defenders(
        board: &ChessBoard,
        king: Square,
        color: Color,
    ) -> usize {
        let mut count = 0;

        for square in Self::adjacent_squares(king) {
            if board.occupied[color.index()].contains(square) {
                count += 1;
            }
        }

        count
    }

    fn pawn_shield(
        board: &ChessBoard,
        king: Square,
        color: Color,
    ) -> usize {
        let mut count = 0;

        let forward = match color {
            Color::White => 1i8,
            Color::Black => -1i8,
        };

        for file_offset in -1i8..=1 {
            let file = king.file() as i8 + file_offset;
            let rank = king.rank() as i8 + forward;

            if !(0..8).contains(&file)
                || !(0..8).contains(&rank)
            {
                continue;
            }

            let square = Square::from_coords(
                file as u8,
                rank as u8,
            ).unwrap();

            if Self::is_piece_at(
                board, color, Piece::Pawn, square
            ) {
                count += 1;
            }
        }

        count
    }

    fn escape_squares(
        board: &ChessBoard,
        king: Square,
        color: Color,
    ) -> usize {
        let enemy = Self::opponent(color);

        Self::adjacent_squares(king)
            .into_iter()
            .filter(|square| {
                !board.occupied[color.index()].contains(*square)
                    && !MoveGenerator::is_square_attacked(
                        board, *square, enemy
                    )
            })
            .count()
    }

    fn enemy_pressure(
        board: &ChessBoard,
        king: Square,
        enemy: Color,
    ) -> usize {
        Self::adjacent_squares(king)
            .into_iter()
            .filter(|square| {
                MoveGenerator::is_square_attacked(
                    board, *square, enemy
                )
            })
            .count()
    }

    fn has_castling_rights(
        state: &GameState,
        color: Color,
    ) -> bool {
        match color {
            Color::White => {
                state.castling.white_kingside
                    || state.castling.white_queenside
            }
            Color::Black => {
                state.castling.black_kingside
                    || state.castling.black_queenside
            }
        }
    }

    fn is_piece_at(
        board: &ChessBoard,
        color: Color,
        piece: Piece,
        square: Square,
    ) -> bool {
        let Some(index) = Piece::ALL
            .iter()
            .position(|p| *p == piece)
        else {
            return false;
        };

        board.pieces[color.index()][index].contains(square)
    }

    fn adjacent_squares(king: Square) -> Vec<Square> {
        let mut squares = Vec::with_capacity(8);

        for dx in -1i8..=1 {
            for dy in -1i8..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let file = king.file() as i8 + dx;
                let rank = king.rank() as i8 + dy;

                if (0..8).contains(&file)
                    && (0..8).contains(&rank)
                {
                    squares.push(
                        Square::from_coords(
                            file as u8,
                            rank as u8,
                        ).unwrap()
                    );
                }
            }
        }

        squares
    }

    fn opponent(color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
