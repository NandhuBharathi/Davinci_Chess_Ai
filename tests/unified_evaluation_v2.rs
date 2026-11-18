
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    unified_evaluation_v2::{
        UnifiedEvaluatorV2,
        EvaluationWeights,
    },
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn basic_board() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));

    board
}

#[test]
fn test_initial_position_is_finite() {
    let board = basic_board();

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    assert!(result.final_score.is_finite());
}

#[test]
fn test_final_score_is_bounded() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    assert!((-1.0..=1.0).contains(&result.final_score));
}

#[test]
fn test_all_normalized_components_are_bounded() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));
    board.add_piece(Color::Black, Piece::Rook, sq(0, 6));
    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    let values = [
        result.normalized.material,
        result.normalized.central_control,
        result.normalized.king_safety,
        result.normalized.pawn_structure,
        result.normalized.pawn_race,
    ];

    for value in values {
        assert!(value.is_finite());
        assert!((-1.0..=1.0).contains(&value));
    }
}

#[test]
fn test_material_advantage_is_included() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    assert!(result.raw.material > 0.0);
    assert!(result.normalized.material > 0.0);
}

#[test]
fn test_black_material_advantage_is_negative() {
    let mut board = basic_board();

    board.add_piece(Color::Black, Piece::Queen, sq(3, 6));

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    assert!(result.raw.material < 0.0);
    assert!(result.normalized.material < 0.0);
}

#[test]
fn test_default_weights_sum_to_one() {
    let weights = EvaluationWeights::default();

    let sum =
        weights.material
        + weights.central_control
        + weights.king_safety
        + weights.pawn_structure
        + weights.pawn_race;

    assert!((sum - 1.0).abs() < 0.0001);
}

#[test]
fn test_custom_material_only_weight() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));

    let weights = EvaluationWeights {
        material: 1.0,
        central_control: 0.0,
        king_safety: 0.0,
        pawn_structure: 0.0,
        pawn_race: 0.0,
    };

    let result = UnifiedEvaluatorV2::evaluate_with_weights(
        &board,
        &GameState::new(),
        weights,
    );

    assert!(
        (result.final_score - result.normalized.material).abs() < 0.0001
    );
}

#[test]
fn test_zero_weights_return_zero() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));

    let weights = EvaluationWeights {
        material: 0.0,
        central_control: 0.0,
        king_safety: 0.0,
        pawn_structure: 0.0,
        pawn_race: 0.0,
    };

    let result = UnifiedEvaluatorV2::evaluate_with_weights(
        &board,
        &GameState::new(),
        weights,
    );

    assert_eq!(result.final_score, 0.0);
}

#[test]
fn test_evaluation_is_deterministic() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let state = GameState::new();

    let first = UnifiedEvaluatorV2::evaluate(&board, &state);
    let second = UnifiedEvaluatorV2::evaluate(&board, &state);

    assert_eq!(first.final_score, second.final_score);
}

#[test]
fn test_all_raw_components_are_finite() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(3, 3));
    board.add_piece(Color::Black, Piece::Rook, sq(0, 6));
    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result =
        UnifiedEvaluatorV2::evaluate(&board, &GameState::new());

    let values = [
        result.raw.material,
        result.raw.central_control,
        result.raw.king_safety,
        result.raw.pawn_structure,
        result.raw.pawn_race,
        result.weighted_score,
        result.final_score,
    ];

    for value in values {
        assert!(value.is_finite());
    }
}
