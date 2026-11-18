
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    unified_evaluation::UnifiedEvaluator,
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
fn test_initial_position_evaluation() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let result = UnifiedEvaluator::evaluate(&board, &state);

    assert!(result.final_score.is_finite());
}

#[test]
fn test_material_advantage_included() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(0, 0));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert_eq!(result.material, 9.0);
}

#[test]
fn test_black_material_advantage() {
    let mut board = basic_board();

    board.add_piece(Color::Black, Piece::Rook, sq(0, 7));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert_eq!(result.material, -5.0);
}

#[test]
fn test_pawn_structure_integration() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert!(result.pawn_structure > 0.0);
}

#[test]
fn test_pawn_race_integration() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 6));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert!(result.pawn_race > 0.0);
}

#[test]
fn test_king_safety_integration() {
    let mut board = basic_board();

    board.add_piece(Color::Black, Piece::Rook, sq(4, 6));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert!(result.king_safety.is_finite());
}

#[test]
fn test_central_control_integration() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 3));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert!(result.central_control > 0.0);
}

#[test]
fn test_final_score_combines_components() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Queen, sq(0, 0));
    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    let expected =
        result.material
        + result.central_control
        + result.king_safety
        + result.pawn_structure
        + result.pawn_race;

    assert!((result.final_score - expected).abs() < 0.0001);
}

#[test]
fn test_evaluation_is_finite() {
    let board = ChessBoard::initial_position();
    let result = UnifiedEvaluator::evaluate(&board, &GameState::new());

    assert!(result.material.is_finite());
    assert!(result.central_control.is_finite());
    assert!(result.king_safety.is_finite());
    assert!(result.pawn_structure.is_finite());
    assert!(result.pawn_race.is_finite());
    assert!(result.final_score.is_finite());
}

#[test]
fn test_repeated_evaluation_is_deterministic() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let first = UnifiedEvaluator::evaluate(&board, &state);
    let second = UnifiedEvaluator::evaluate(&board, &state);

    assert_eq!(first.final_score, second.final_score);
}
