
use crate::{
    ChessBoard, Color, Piece, Square,
    move_generator::MoveGenerator,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct EvaluationBreakdown {
    pub material: f32,
    pub central_control: f32,
    pub total: f32,
}

pub struct PositionEvaluation;

impl PositionEvaluation {
    const CENTER_BONUS: f32 = 0.15;
    const CENTER_ATTACK_BONUS: f32 = 0.10;

    pub fn evaluate(board: &ChessBoard) -> EvaluationBreakdown {
        let material = Self::material_score(board);
        let central_control = Self::central_control_score(board);

        EvaluationBreakdown {
            material,
            central_control,
            total: material + central_control,
        }
    }

    // Positive = White advantage
    // Negative = Black advantage
    fn material_score(board: &ChessBoard) -> f32 {
        let mut score = 0.0;

        for color in [Color::White, Color::Black] {
            let sign = if color == Color::White {
                1.0
            } else {
                -1.0
            };

            for piece in Piece::ALL {
                let value = match piece {
                    Piece::King => 0.0,
                    Piece::Queen => 9.0,
                    Piece::Rook => 5.0,
                    Piece::Bishop => 3.0,
                    Piece::Knight => 3.0,
                    Piece::Pawn => 1.0,
                };

                score +=
                    sign * value * board.piece_count(color, piece) as f32;
            }
        }

        score
    }

    fn central_control_score(board: &ChessBoard) -> f32 {
        let center = [
            (3, 3), // d4
            (4, 3), // e4
            (3, 4), // d5
            (4, 4), // e5
        ];

        let mut score = 0.0;

        for (file, rank) in center {
            let Some(square) = Square::from_coords(file, rank) else {
                continue;
            };

            // Occupying the center
            if board.occupied[Color::White.index()].contains(square) {
                score += Self::CENTER_BONUS;
            }

            if board.occupied[Color::Black.index()].contains(square) {
                score -= Self::CENTER_BONUS;
            }

            // Attacking / controlling the center
            let white_controls =
                MoveGenerator::is_square_attacked(
                    board, square, Color::White
                );

            let black_controls =
                MoveGenerator::is_square_attacked(
                    board, square, Color::Black
                );

            if white_controls {
                score += Self::CENTER_ATTACK_BONUS;
            }

            if black_controls {
                score -= Self::CENTER_ATTACK_BONUS;
            }
        }

        score
    }
}
