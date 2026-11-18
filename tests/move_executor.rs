
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    move_executor::MoveExecutor,
};

#[test]
fn test_normal_pawn_move() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(12), // E2
        to: Square(28),   // E4
    };

    let (next, next_state) =
        MoveExecutor::execute(&board, &state, mv).unwrap();

    assert!(next.pieces[Color::White.index()]
        [Piece::Pawn as usize].contains(Square(28)));

    assert_eq!(next.side_to_move, Color::Black);
    assert_eq!(next_state.en_passant_target, Some(20));
    assert_eq!(next_state.halfmove_clock, 0);
    assert_eq!(next_state.ply, 1);
}

#[test]
fn test_black_double_pawn_move() {
    let mut board = ChessBoard::initial_position();
    board.side_to_move = Color::Black;

    let state = GameState::default();

    let mv = ChessMove {
        from: Square(52), // E7
        to: Square(36),   // E5
    };

    let (next, next_state) =
        MoveExecutor::execute(&board, &state, mv).unwrap();

    assert_eq!(next.side_to_move, Color::White);
    assert_eq!(next_state.en_passant_target, Some(44));
    assert_eq!(next_state.fullmove_number, 2);
}

#[test]
fn test_invalid_move_rejected() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let mv = ChessMove {
        from: Square(12), // E2
        to: Square(36),   // E5 (illegal)
    };

    assert!(MoveExecutor::execute(&board, &state, mv).is_none());
}

#[test]
fn test_original_position_unchanged() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let original_board = board.clone();
    let original_ply = state.ply;

    let mv = ChessMove {
        from: Square(12),
        to: Square(28),
    };

    let _ = MoveExecutor::execute(&board, &state, mv);

    assert_eq!(
        board.all_occupied.0,
        original_board.all_occupied.0
    );

    assert_eq!(state.ply, original_ply);
}

#[test]
fn test_en_passant_execution() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::Black, Piece::King, Square(60));

    board.add_piece(Color::White, Piece::Pawn, Square(36));
    board.add_piece(Color::Black, Piece::Pawn, Square(35));

    board.side_to_move = Color::White;

    let mut state = GameState::default();
    state.en_passant_target = Some(43);

    let mv = ChessMove {
        from: Square(36),
        to: Square(43),
    };

    let (next, next_state) =
        MoveExecutor::execute(&board, &state, mv).unwrap();

    assert!(next.pieces[Color::White.index()]
        [Piece::Pawn as usize].contains(Square(43)));

    assert!(!next.pieces[Color::Black.index()]
        [Piece::Pawn as usize].contains(Square(35)));

    assert_eq!(next.side_to_move, Color::Black);
    assert_eq!(next_state.en_passant_target, None);
    assert_eq!(next_state.halfmove_clock, 0);
}
