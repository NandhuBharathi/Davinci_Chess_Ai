
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    pawn_race::PawnRace,
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn board() -> ChessBoard {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(0, 0));
    b.add_piece(Color::Black, Piece::King, sq(7, 7));

    b
}

#[test]
fn test_white_passed_pawn_detected() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert_eq!(r.white.passed_pawns, 1);
}

#[test]
fn test_black_passed_pawn_detected() {
    let mut b = board();
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 2));

    let r = PawnRace::evaluate(&b);

    assert_eq!(r.black.passed_pawns, 1);
}

#[test]
fn test_blocked_pawn_detected() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 4));
    b.add_piece(Color::Black, Piece::Rook, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert_eq!(r.white.blocked_pawns, 1);
}

#[test]
fn test_promotion_distance() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 6));

    let r = PawnRace::evaluate(&b);

    assert!(r.white.promotion_distance > 0.0);
}

#[test]
fn test_advanced_pawn_has_urgency() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 6));

    let r = PawnRace::evaluate(&b);

    assert!(r.white.urgency > 0.0);
}

#[test]
fn test_friendly_king_support() {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(2, 6));
    b.add_piece(Color::Black, Piece::King, sq(7, 7));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert!(r.white.king_support > 0.0);
}

#[test]
fn test_enemy_king_interception() {
    let mut b = ChessBoard::empty();

    b.add_piece(Color::White, Piece::King, sq(0, 0));
    b.add_piece(Color::Black, Piece::King, sq(4, 6));
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert!(r.white.interception_risk > 0.0);
}

#[test]
fn test_no_passed_pawn() {
    let mut b = board();

    b.add_piece(Color::White, Piece::Pawn, sq(3, 3));
    b.add_piece(Color::Black, Piece::Pawn, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert_eq!(r.white.passed_pawns, 0);
}

#[test]
fn test_total_score_consistency() {
    let mut b = board();

    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));
    b.add_piece(Color::Black, Piece::Pawn, sq(4, 2));

    let r = PawnRace::evaluate(&b);

    assert!(
        (r.total - (r.white.total - r.black.total)).abs() < 0.0001
    );
}

#[test]
fn test_score_is_finite() {
    let mut b = board();
    b.add_piece(Color::White, Piece::Pawn, sq(3, 5));

    let r = PawnRace::evaluate(&b);

    assert!(r.total.is_finite());
}
