
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    king_safety::KingSafety,
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
fn test_initial_position_safety_is_finite() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let result = KingSafety::evaluate(&board, &state);

    assert!(result.total.is_finite());
    assert!(result.white_score.is_finite());
    assert!(result.black_score.is_finite());
}

#[test]
fn test_white_king_in_check_penalty() {
    let mut board = basic_board();

    board.add_piece(Color::Black, Piece::Rook, sq(4, 6));

    let result = KingSafety::evaluate(&board, &GameState::new());

    assert!(result.white_score < 0.0);
}

#[test]
fn test_black_king_in_check_penalty() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Rook, sq(4, 1));

    let result = KingSafety::evaluate(&board, &GameState::new());

    assert!(result.black_score < 0.0);
}

#[test]
fn test_white_pawn_shield_bonus() {
    let mut board = basic_board();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 1));
    board.add_piece(Color::White, Piece::Pawn, sq(4, 1));
    board.add_piece(Color::White, Piece::Pawn, sq(5, 1));

    let result = KingSafety::evaluate(&board, &GameState::new());

    assert!(result.white_score > 0.0);
}

#[test]
fn test_black_pawn_shield_bonus() {
    let mut board = basic_board();

    board.add_piece(Color::Black, Piece::Pawn, sq(3, 6));
    board.add_piece(Color::Black, Piece::Pawn, sq(4, 6));
    board.add_piece(Color::Black, Piece::Pawn, sq(5, 6));

    let result = KingSafety::evaluate(&board, &GameState::new());

    assert!(result.black_score > 0.0);
}

#[test]
fn test_castling_rights_bonus() {
    let board = basic_board();

    let mut state = GameState::new();
    state.castling.white_kingside = false;
    state.castling.white_queenside = false;

    let without_rights = KingSafety::evaluate(&board, &state);

    state.castling.white_kingside = true;

    let with_rights = KingSafety::evaluate(&board, &state);

    assert!(with_rights.white_score > without_rights.white_score);
}

#[test]
fn test_white_and_black_scores_are_separate() {
    let board = basic_board();
    let result = KingSafety::evaluate(&board, &GameState::new());

    assert!(result.white_score.is_finite());
    assert!(result.black_score.is_finite());
}

#[test]
fn test_total_score_calculation() {
    let board = ChessBoard::initial_position();
    let result = KingSafety::evaluate(&board, &GameState::new());

    let expected = result.white_score - result.black_score;

    assert!((result.total - expected).abs() < 0.0001);
}
