
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    move_executor::{MoveExecutor, PromotionChoice},
};

fn white_promotion_board() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::Black, Piece::King, Square(60));
    board.add_piece(Color::White, Piece::Pawn, Square(48));

    board.side_to_move = Color::White;
    board
}

fn black_promotion_board() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::Black, Piece::King, Square(60));
    board.add_piece(Color::Black, Piece::Pawn, Square(8));

    board.side_to_move = Color::Black;
    board
}

#[test]
fn test_white_pawn_promotes_to_queen() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    let (next, next_state) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Queen
    ).unwrap();

    assert!(next.pieces[0][Piece::Queen as usize].contains(Square(56)));
    assert!(!next.pieces[0][Piece::Pawn as usize].contains(Square(56)));
    assert_eq!(next.side_to_move, Color::Black);
    assert_eq!(next_state.halfmove_clock, 0);
}

#[test]
fn test_white_promotion_to_rook() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    let (next, _) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Rook
    ).unwrap();

    assert!(next.pieces[0][Piece::Rook as usize].contains(Square(56)));
}

#[test]
fn test_white_promotion_to_bishop() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    let (next, _) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Bishop
    ).unwrap();

    assert!(next.pieces[0][Piece::Bishop as usize].contains(Square(56)));
}

#[test]
fn test_white_promotion_to_knight() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    let (next, _) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Knight
    ).unwrap();

    assert!(next.pieces[0][Piece::Knight as usize].contains(Square(56)));
}

#[test]
fn test_black_pawn_promotes_to_queen() {
    let board = black_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(8),
        to: Square(0),
    };

    let (next, next_state) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Queen
    ).unwrap();

    assert!(next.pieces[1][Piece::Queen as usize].contains(Square(0)));
    assert_eq!(next.side_to_move, Color::White);
    assert_eq!(next_state.fullmove_number, 2);
}

#[test]
fn test_promotion_capture() {
    let mut board = white_promotion_board();

    board.add_piece(Color::Black, Piece::Rook, Square(57));

    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(57),
    };

    let (next, next_state) = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Knight
    ).unwrap();

    assert!(next.pieces[0][Piece::Knight as usize].contains(Square(57)));
    assert!(!next.pieces[1][Piece::Rook as usize].contains(Square(57)));
    assert_eq!(next_state.halfmove_clock, 0);
}

#[test]
fn test_normal_executor_rejects_unpromoted_pawn() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    assert!(MoveExecutor::execute(&board, &state, mv).is_none());
}

#[test]
fn test_promotion_rejects_non_final_rank() {
    let board = white_promotion_board();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(48),
        to: Square(40),
    };

    assert!(MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Queen
    ).is_none());
}

#[test]
fn test_promotion_does_not_modify_original_board() {
    let board = white_promotion_board();
    let state = GameState::default();

    let original = board.clone();

    let mv = ChessMove {
        from: Square(48),
        to: Square(56),
    };

    let _ = MoveExecutor::execute_promotion(
        &board, &state, mv, PromotionChoice::Queen
    );

    assert_eq!(board.all_occupied.0, original.all_occupied.0);
    assert_eq!(board.side_to_move, original.side_to_move);
}
