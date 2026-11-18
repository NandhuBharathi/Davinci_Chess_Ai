
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    pawn_structure::PawnStructure,
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn empty_board() -> ChessBoard {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(0, 0));
    b.add_piece(Color::Black, Piece::King, sq(7, 7));

    b
}

#[test]
fn test_white_backward_pawn() {
    let mut b = empty_board();

    // White pawn e4
    b.add_piece(Color::White, Piece::Pawn, sq(4, 3));

    // Black pawn d6 attacks e5
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&b);

    assert!(result.white.backward < 0.0);
}

#[test]
fn test_black_backward_pawn() {
    let mut b = empty_board();

    // Black pawn e5
    b.add_piece(Color::Black, Piece::Pawn, sq(4, 4));

    // White pawn d3 attacks e4
    b.add_piece(Color::White, Piece::Pawn, sq(3, 2));

    let result = PawnStructure::evaluate(&b);

    assert!(result.black.backward < 0.0);
}

#[test]
fn test_supported_pawn_is_not_backward() {
    let mut b = empty_board();

    // White pawn e4
    b.add_piece(Color::White, Piece::Pawn, sq(4, 3));

    // Black pawn d6 attacks e5
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    // White pawn d4 protects e5
    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));

    let result = PawnStructure::evaluate(&b);

    assert_eq!(result.white.backward, 0.0);
}

#[test]
fn test_friendly_king_closer_to_passed_pawn() {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(3, 4));
    b.add_piece(Color::Black, Piece::King, sq(7, 7));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&b);

    assert!(result.white.king_distance > 0.0);
}

#[test]
fn test_enemy_king_closer_to_passed_pawn() {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(0, 0));
    b.add_piece(Color::Black, Piece::King, sq(3, 6));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&b);

    assert!(result.white.king_distance < 0.0);
}

#[test]
fn test_no_passed_pawn_no_king_distance_score() {
    let mut b = empty_board();

    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    let result = PawnStructure::evaluate(&b);

    assert_eq!(result.white.king_distance, 0.0);
}
