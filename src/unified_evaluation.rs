
use crate::{
    ChessBoard,
    game_state::GameState,
    position_evaluation::PositionEvaluation,
    king_safety::KingSafety,
    pawn_structure::PawnStructure,
    pawn_race::PawnRace,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct UnifiedEvaluation {
    pub material: f32,
    pub central_control: f32,
    pub king_safety: f32,
    pub pawn_structure: f32,
    pub pawn_race: f32,
    pub final_score: f32,
}

pub struct UnifiedEvaluator;

impl UnifiedEvaluator {
    // Initial heuristic weights.
    // These can later be tuned through self-play training.
    const MATERIAL_WEIGHT: f32 = 1.0;
    const CENTER_WEIGHT: f32 = 1.0;
    const KING_SAFETY_WEIGHT: f32 = 1.0;
    const PAWN_STRUCTURE_WEIGHT: f32 = 1.0;
    const PAWN_RACE_WEIGHT: f32 = 1.0;

    pub fn evaluate(
        board: &ChessBoard,
        state: &GameState,
    ) -> UnifiedEvaluation {

        // 1. Material and central control
        let position = PositionEvaluation::evaluate(board);

        // 2. King safety
        let king = KingSafety::evaluate(board, state);

        // 3. Pawn structure V2
        let pawns = PawnStructure::evaluate(board);

        // 4. Pawn promotion race V3
        let race = PawnRace::evaluate(board);

        let mut result = UnifiedEvaluation {
            material: position.material,
            central_control: position.central_control,
            king_safety: king.total,
            pawn_structure: pawns.total,
            pawn_race: race.total,
            final_score: 0.0,
        };

        result.final_score =
            result.material * Self::MATERIAL_WEIGHT
            + result.central_control * Self::CENTER_WEIGHT
            + result.king_safety * Self::KING_SAFETY_WEIGHT
            + result.pawn_structure * Self::PAWN_STRUCTURE_WEIGHT
            + result.pawn_race * Self::PAWN_RACE_WEIGHT;

        result
    }
}
