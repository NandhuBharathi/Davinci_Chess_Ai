
use crate::{ChessBoard, Color, Piece, Square};

#[derive(Debug, Clone, Copy, Default)]
pub struct PawnStructureBreakdown {
    pub white_score: f32,
    pub black_score: f32,
    pub total: f32,
}

pub struct PawnStructure;

impl PawnStructure {
    const DOUBLED_PENALTY: f32 = 0.50;
    const ISOLATED_PENALTY: f32 = 0.50;
    const CONNECTED_BONUS: f32 = 0.25;
    const PASSED_BASE_BONUS: f32 = 0.40;
    const PASSED_ADVANCEMENT_BONUS: f32 = 0.15;

    pub fn evaluate(board: &ChessBoard) -> PawnStructureBreakdown {
        let white_score = Self::evaluate_side(board, Color::White);
        let black_score = Self::evaluate_side(board, Color::Black);

        PawnStructureBreakdown {
            white_score,
            black_score,
            total: white_score - black_score,
        }
    }

    fn evaluate_side(board: &ChessBoard, color: Color) -> f32 {
        let pawns = Self::get_pawns(board, color);
        let enemy_pawns = Self::get_pawns(
            board,
            Self::opponent(color),
        );

        if pawns.is_empty() {
            return 0.0;
        }

        let mut score = 0.0;

        // 1. Doubled pawns
        let mut file_counts = [0usize; 8];

        for pawn in &pawns {
            file_counts[pawn.file() as usize] += 1;
        }

        for count in file_counts {
            if count > 1 {
                score -= (count - 1) as f32
                    * Self::DOUBLED_PENALTY;
            }
        }

        // 2. Isolated pawns
        for pawn in &pawns {
            let file = pawn.file() as i8;

            let has_neighboring_file_pawn = pawns.iter().any(|other| {
                let other_file = other.file() as i8;

                (other_file - file).abs() == 1
            });

            if !has_neighboring_file_pawn {
                score -= Self::ISOLATED_PENALTY;
            }
        }

        // 3. Connected pawns
        for i in 0..pawns.len() {
            for j in (i + 1)..pawns.len() {
                let file_diff = (
                    pawns[i].file() as i8
                    - pawns[j].file() as i8
                ).abs();

                let rank_diff = (
                    pawns[i].rank() as i8
                    - pawns[j].rank() as i8
                ).abs();

                if file_diff == 1 && rank_diff <= 1 {
                    score += Self::CONNECTED_BONUS;
                }
            }
        }

        // 4. Passed pawns
        for pawn in &pawns {
            if Self::is_passed_pawn(*pawn, &enemy_pawns, color) {
                let advancement = match color {
                    Color::White => pawn.rank() as f32,
                    Color::Black => (7 - pawn.rank()) as f32,
                };

                score += Self::PASSED_BASE_BONUS
                    + advancement * Self::PASSED_ADVANCEMENT_BONUS;
            }
        }

        score
    }

    fn get_pawns(board: &ChessBoard, color: Color) -> Vec<Square> {
        let Some(index) = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::Pawn)
        else {
            return Vec::new();
        };

        let mut bitboard = board.pieces[color.index()][index];
        let mut pawns = Vec::new();

        while let Some(square) = bitboard.pop_lsb() {
            pawns.push(square);
        }

        pawns
    }

    fn is_passed_pawn(
        pawn: Square,
        enemy_pawns: &[Square],
        color: Color,
    ) -> bool {
        let file = pawn.file() as i8;
        let rank = pawn.rank() as i8;

        !enemy_pawns.iter().any(|enemy| {
            let enemy_file = enemy.file() as i8;
            let enemy_rank = enemy.rank() as i8;

            let same_or_adjacent_file =
                (enemy_file - file).abs() <= 1;

            let ahead = match color {
                Color::White => enemy_rank > rank,
                Color::Black => enemy_rank < rank,
            };

            same_or_adjacent_file && ahead
        })
    }

    fn opponent(color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
