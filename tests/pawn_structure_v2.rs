
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    pawn_structure::PawnStructure,
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn board() -> ChessBoard {
    let mut b = ChessBoard::empty();
    b.add_piece(Color::White, Piece::King, sq(4, 0));
    b.add_piece(Color::Black, Piece::King, sq(4, 7));
    b
}

#[test]
fn test_doubled_pawn_penalty() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 2));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.doubled < 0.0);
}

#[test]
fn test_isolated_pawn_penalty() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(0, 3));
    b.add_piece(Color::Black, Piece::Pawn, sq(0, 5));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.isolated < 0.0);
}

#[test]
fn test_pawn_islands() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(0, 2));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 2));
    b.add_piece(Color::White, Piece::Pawn, sq(6, 2));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.islands < 0.0);
}

#[test]
fn test_connected_pawns() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::White, Piece::Pawn, sq(4, 3));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.connected > 0.0);
}

#[test]
fn test_protected_pawn() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::White, Piece::Pawn, sq(2, 2));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.protected > 0.0);
}

#[test]
fn test_passed_pawn() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.passed > 0.0);
}

#[test]
fn test_blocked_passed_pawn_path() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::Black, Piece::Rook, sq(3, 5));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.passed > 0.0);
    assert!(r.white.passed < 0.50);
}

#[test]
fn test_passed_pawn_advancement() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnStructure::evaluate(&b);
    assert!(r.white.advancement > 0.0);
}

#[test]
fn test_black_pawn_evaluation() {
    let mut b = board();
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 2));

    let r = PawnStructure::evaluate(&b);
    assert!(r.black.passed > 0.0);
}

#[test]
fn test_total_score_consistency() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));
    b.add_piece(Color::Black, Piece::Pawn, sq(4, 2));

    let r = PawnStructure::evaluate(&b);

    assert!(
        (r.total - (r.white.total - r.black.total)).abs() < 0.0001
    );
}
