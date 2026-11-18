
use crate::{ChessBoard, Color, Piece, Square};

#[derive(Debug, Clone, Copy, Default)]
pub struct PawnRaceBreakdown {
    pub passed_pawns: usize,
    pub blocked_pawns: usize,
    pub promotion_distance: f32,
    pub king_support: f32,
    pub interception_risk: f32,
    pub urgency: f32,
    pub total: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PawnRaceEvaluation {
    pub white: PawnRaceBreakdown,
    pub black: PawnRaceBreakdown,
    pub total: f32,
}

pub struct PawnRace;

impl PawnRace {
    const URGENCY_WEIGHT: f32 = 0.30;
    const KING_SUPPORT_WEIGHT: f32 = 0.20;
    const INTERCEPTION_PENALTY: f32 = 1.0;
    const BLOCKED_PENALTY: f32 = 0.75;

    pub fn evaluate(board: &ChessBoard) -> PawnRaceEvaluation {
        let white = Self::evaluate_side(board, Color::White);
        let black = Self::evaluate_side(board, Color::Black);

        PawnRaceEvaluation {
            total: white.total - black.total,
            white,
            black,
        }
    }

    fn evaluate_side(
        board: &ChessBoard,
        color: Color,
    ) -> PawnRaceBreakdown {
        let pawns = Self::get_pawns(board, color);
        let enemy_pawns = Self::get_pawns(
            board,
            Self::opponent(color),
        );

        let mut result = PawnRaceBreakdown::default();

        for pawn in pawns {
            if !Self::is_passed(pawn, &enemy_pawns, color) {
                continue;
            }

            result.passed_pawns += 1;

            let distance = Self::promotion_distance(pawn, color);

            let blocked = Self::path_blocked(
                board, pawn, color
            );

            if blocked {
                result.blocked_pawns += 1;
                result.total -= Self::BLOCKED_PENALTY;
                continue;
            }

            let moves = Self::moves_to_promotion(
                board, pawn, color, distance
            );

            result.promotion_distance += moves as f32;

            // Promotion urgency: fewer moves means higher urgency.
            let urgency = (7.0 - moves as f32).max(0.0);

            result.urgency +=
                urgency * Self::URGENCY_WEIGHT;

            // Friendly King support
            let friendly_distance = Self::king_distance_to(
                board, color, Self::promotion_square(pawn, color)
            );

            if let Some(distance) = friendly_distance {
                let support =
                    (7.0 - distance as f32).max(0.0)
                    * Self::KING_SUPPORT_WEIGHT;

                result.king_support += support;
            }

            // Enemy King interception risk
            let enemy_distance = Self::king_distance_to(
                board,
                Self::opponent(color),
                Self::promotion_square(pawn, color)
            );

            if let Some(distance) = enemy_distance {
                if distance <= moves {
                    result.interception_risk +=
                        Self::INTERCEPTION_PENALTY;
                }
            }
        }

        result.total =
            result.urgency
            + result.king_support
            - result.interception_risk
            - result.blocked_pawns as f32 * Self::BLOCKED_PENALTY;

        result
    }

    fn get_pawns(
        board: &ChessBoard,
        color: Color,
    ) -> Vec<Square> {
        let index = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::Pawn)
            .unwrap();

        let mut bits = board.pieces[color.index()][index];
        let mut result = Vec::new();

        while let Some(square) = bits.pop_lsb() {
            result.push(square);
        }

        result
    }

    fn is_passed(
        pawn: Square,
        enemy_pawns: &[Square],
        color: Color,
    ) -> bool {
        let file = pawn.file() as i8;
        let rank = pawn.rank() as i8;

        !enemy_pawns.iter().any(|enemy| {
            let enemy_file = enemy.file() as i8;
            let enemy_rank = enemy.rank() as i8;

            let same_or_adjacent =
                (enemy_file - file).abs() <= 1;

            let ahead = match color {
                Color::White => enemy_rank > rank,
                Color::Black => enemy_rank < rank,
            };

            same_or_adjacent && ahead
        })
    }

    fn promotion_distance(
        pawn: Square,
        color: Color,
    ) -> u8 {
        match color {
            Color::White => 7 - pawn.rank(),
            Color::Black => pawn.rank(),
        }
    }

    fn promotion_square(
        pawn: Square,
        color: Color,
    ) -> Square {
        let rank = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        Square::from_coords(pawn.file(), rank).unwrap()
    }

    fn path_blocked(
        board: &ChessBoard,
        pawn: Square,
        color: Color,
    ) -> bool {
        let direction = match color {
            Color::White => 1i8,
            Color::Black => -1i8,
        };

        let mut rank = pawn.rank() as i8 + direction;

        while (0..8).contains(&rank) {
            let square = Square::from_coords(
                pawn.file(),
                rank as u8,
            ).unwrap();

            if board.all_occupied.contains(square) {
                return true;
            }

            rank += direction;
        }

        false
    }

    fn moves_to_promotion(
        board: &ChessBoard,
        pawn: Square,
        color: Color,
        distance: u8,
    ) -> u8 {
        if distance == 0 {
            return 0;
        }

        let mut moves = distance;

        // A pawn on its starting rank may double-step.
        let starting_rank = match color {
            Color::White => 1,
            Color::Black => 6,
        };

        if pawn.rank() == starting_rank && distance >= 2 {
            let middle_rank = match color {
                Color::White => 2,
                Color::Black => 5,
            };

            let middle = Square::from_coords(
                pawn.file(), middle_rank
            ).unwrap();

            if !board.all_occupied.contains(middle) {
                moves -= 1;
            }
        }

        // If the pawn's side is not to move,
        // the opponent gets the first tempo.
        if board.side_to_move != color {
            moves = moves.saturating_mul(2).saturating_sub(1);
        } else {
            moves = moves.saturating_mul(2);
        }

        moves
    }

    fn king_distance_to(
        board: &ChessBoard,
        color: Color,
        target: Square,
    ) -> Option<u8> {
        let index = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::King)?;

        let mut bits = board.pieces[color.index()][index];
        let king = bits.pop_lsb()?;

        let dx = (king.file() as i8 - target.file() as i8).abs();
        let dy = (king.rank() as i8 - target.rank() as i8).abs();

        Some(dx.max(dy) as u8)
    }

    fn opponent(color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
