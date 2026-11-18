
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    position_evaluation::PositionEvaluation,
};

fn empty_board() -> ChessBoard {
    ChessBoard::empty()
}

#[test]
fn test_equal_material_is_zero() {
    let board = ChessBoard::initial_position();
    let result = PositionEvaluation::evaluate(&board);

    assert_eq!(result.material, 0.0);
}

#[test]
fn test_white_queen_advantage() {
    let mut board = empty_board();

    board.add_piece(Color::White, Piece::King, Square::from_coords(4, 0).unwrap());
    board.add_piece(Color::Black, Piece::King, Square::from_coords(4, 7).unwrap());
    board.add_piece(Color::White, Piece::Queen, Square::from_coords(3, 3).unwrap());

    let result = PositionEvaluation::evaluate(&board);

    assert_eq!(result.material, 9.0);
}

#[test]
fn test_black_rook_advantage() {
    let mut board = empty_board();

    board.add_piece(Color::White, Piece::King, Square::from_coords(4, 0).unwrap());
    board.add_piece(Color::Black, Piece::King, Square::from_coords(4, 7).unwrap());
    board.add_piece(Color::Black, Piece::Rook, Square::from_coords(0, 7).unwrap());

    let result = PositionEvaluation::evaluate(&board);

    assert_eq!(result.material, -5.0);
}

#[test]
fn test_material_values() {
    let mut board = empty_board();

    board.add_piece(Color::White, Piece::King, Square::from_coords(4, 0).unwrap());
    board.add_piece(Color::Black, Piece::King, Square::from_coords(4, 7).unwrap());
    board.add_piece(Color::White, Piece::Queen, Square::from_coords(0, 0).unwrap());
    board.add_piece(Color::White, Piece::Rook, Square::from_coords(1, 0).unwrap());
    board.add_piece(Color::White, Piece::Bishop, Square::from_coords(2, 0).unwrap());
    board.add_piece(Color::White, Piece::Knight, Square::from_coords(3, 0).unwrap());
    board.add_piece(Color::White, Piece::Pawn, Square::from_coords(0, 1).unwrap());

    let result = PositionEvaluation::evaluate(&board);

    assert_eq!(result.material, 21.0);
}

#[test]
fn test_white_center_occupancy_bonus() {
    let mut board = empty_board();

    board.add_piece(Color::White, Piece::King, Square::from_coords(0, 0).unwrap());
    board.add_piece(Color::Black, Piece::King, Square::from_coords(7, 7).unwrap());
    board.add_piece(Color::White, Piece::Pawn, Square::from_coords(3, 3).unwrap());

    let result = PositionEvaluation::evaluate(&board);

    assert!(result.central_control > 0.0);
}

#[test]
fn test_black_center_occupancy_bonus() {
    let mut board = empty_board();

    board.add_piece(Color::White, Piece::King, Square::from_coords(0, 0).unwrap());
    board.add_piece(Color::Black, Piece::King, Square::from_coords(7, 7).unwrap());
    board.add_piece(Color::Black, Piece::Pawn, Square::from_coords(4, 4).unwrap());

    let result = PositionEvaluation::evaluate(&board);

    assert!(result.central_control < 0.0);
}

#[test]
fn test_total_score_combines_features() {
    let board = ChessBoard::initial_position();
    let result = PositionEvaluation::evaluate(&board);

    assert!(
        (result.total - (result.material + result.central_control)).abs()
            < 0.0001
    );
}

#[test]
fn test_score_is_finite() {
    let board = ChessBoard::initial_position();
    let result = PositionEvaluation::evaluate(&board);

    assert!(result.total.is_finite());
}
