
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    pawn_structure::PawnStructure,
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn board_with_kings() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));

    board
}

#[test]
fn test_empty_pawn_structure() {
    let board = board_with_kings();
    let result = PawnStructure::evaluate(&board);

    assert_eq!(result.white.total, 0.0);
    assert_eq!(result.black.total, 0.0);
    assert_eq!(result.total, 0.0);
}

#[test]
fn test_doubled_pawn_penalty() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 2));
    board.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    board.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&board);

    assert!(result.white.doubled < 0.0);
}

#[test]
fn test_isolated_pawn_penalty() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(0, 3));
    board.add_piece(Color::Black, Piece::Pawn, sq(0, 5));

    let result = PawnStructure::evaluate(&board);

    assert!(result.white.isolated < 0.0);
}

#[test]
fn test_connected_pawn_bonus() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    board.add_piece(Color::White, Piece::Pawn, sq(4, 3));

    let result = PawnStructure::evaluate(&board);

    assert!(result.white.connected > 0.0);
}

#[test]
fn test_white_passed_pawn_bonus() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&board);

    assert!(result.white.passed > 0.0);
}

#[test]
fn test_black_passed_pawn_bonus() {
    let mut board = board_with_kings();

    board.add_piece(Color::Black, Piece::Pawn, sq(3, 2));

    let result = PawnStructure::evaluate(&board);

    assert!(result.black.passed > 0.0);
}

#[test]
fn test_enemy_pawn_blocks_passed_pawn() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 2));
    board.add_piece(Color::Black, Piece::Pawn, sq(3, 4));

    let result = PawnStructure::evaluate(&board);

    assert!(result.white.passed <= 0.0);
}

#[test]
fn test_total_score_calculation() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Pawn, sq(3, 5));
    board.add_piece(Color::Black, Piece::Pawn, sq(4, 2));

    let result = PawnStructure::evaluate(&board);

    assert!(
        (result.total - (result.white.total - result.black.total)).abs()
            < 0.0001
    );
}
