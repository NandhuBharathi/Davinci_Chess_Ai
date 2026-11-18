
use crate::{
    ChessBoard,
    game_state::GameState,
    position_evaluation::PositionEvaluation,
    king_safety::KingSafety,
    pawn_structure::PawnStructure,
    pawn_race::PawnRace,
};

#[derive(Debug, Clone, Copy)]
pub struct EvaluationWeights {
    pub material: f32,
    pub central_control: f32,
    pub king_safety: f32,
    pub pawn_structure: f32,
    pub pawn_race: f32,
}

impl Default for EvaluationWeights {
    fn default() -> Self {
        Self {
            material: 0.45,
            central_control: 0.10,
            king_safety: 0.15,
            pawn_structure: 0.15,
            pawn_race: 0.15,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RawEvaluation {
    pub material: f32,
    pub central_control: f32,
    pub king_safety: f32,
    pub pawn_structure: f32,
    pub pawn_race: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedEvaluation {
    pub material: f32,
    pub central_control: f32,
    pub king_safety: f32,
    pub pawn_structure: f32,
    pub pawn_race: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct UnifiedEvaluationV2 {
    pub raw: RawEvaluation,
    pub normalized: NormalizedEvaluation,
    pub weighted_score: f32,
    pub final_score: f32,
}

pub struct UnifiedEvaluatorV2;

impl UnifiedEvaluatorV2 {
    // Smoothly maps values into [-1, +1].
    fn normalize(value: f32, scale: f32) -> f32 {
        if !value.is_finite() {
            return 0.0;
        }

        (value / scale).tanh()
    }

    pub fn evaluate(
        board: &ChessBoard,
        state: &GameState,
    ) -> UnifiedEvaluationV2 {
        Self::evaluate_with_weights(
            board,
            state,
            EvaluationWeights::default(),
        )
    }

    pub fn evaluate_with_weights(
        board: &ChessBoard,
        state: &GameState,
        weights: EvaluationWeights,
    ) -> UnifiedEvaluationV2 {
        let position = PositionEvaluation::evaluate(board);
        let king = KingSafety::evaluate(board, state);
        let pawn_structure = PawnStructure::evaluate(board);
        let pawn_race = PawnRace::evaluate(board);

        let raw = RawEvaluation {
            material: position.material,
            central_control: position.central_control,
            king_safety: king.total,
            pawn_structure: pawn_structure.total,
            pawn_race: pawn_race.total,
        };

        let normalized = NormalizedEvaluation {
            material: Self::normalize(raw.material, 10.0),
            central_control: Self::normalize(raw.central_control, 1.5),
            king_safety: Self::normalize(raw.king_safety, 2.0),
            pawn_structure: Self::normalize(raw.pawn_structure, 4.0),
            pawn_race: Self::normalize(raw.pawn_race, 3.0),
        };

        let weighted_score =
            normalized.material * weights.material
            + normalized.central_control * weights.central_control
            + normalized.king_safety * weights.king_safety
            + normalized.pawn_structure * weights.pawn_structure
            + normalized.pawn_race * weights.pawn_race;

        let weight_sum =
            weights.material
            + weights.central_control
            + weights.king_safety
            + weights.pawn_structure
            + weights.pawn_race;

        let final_score = if weight_sum > 0.0 && weight_sum.is_finite() {
            (weighted_score / weight_sum).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        UnifiedEvaluationV2 {
            raw,
            normalized,
            weighted_score,
            final_score,
        }
    }
}
