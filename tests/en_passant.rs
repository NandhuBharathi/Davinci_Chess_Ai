
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    en_passant::EnPassant,
};

fn sq(index: u8) -> Square {
    Square(index)
}

fn white_ep_position() -> (ChessBoard, GameState) {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4));
    board.add_piece(Color::Black, Piece::King, sq(60));

    board.add_piece(Color::White, Piece::Pawn, sq(36)); // E5
    board.add_piece(Color::Black, Piece::Pawn, sq(35)); // D5

    board.side_to_move = Color::White;

    let mut state = GameState::default();
    state.en_passant_target = Some(43); // D6

    (board, state)
}

fn black_ep_position() -> (ChessBoard, GameState) {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4));
    board.add_piece(Color::Black, Piece::King, sq(60));

    board.add_piece(Color::Black, Piece::Pawn, sq(28)); // E4
    board.add_piece(Color::White, Piece::Pawn, sq(27)); // D4

    board.side_to_move = Color::Black;

    let mut state = GameState::default();
    state.en_passant_target = Some(19); // D3

    (board, state)
}

#[test]
fn test_white_en_passant_capture() {
    let (board, state) = white_ep_position();

    let mv = ChessMove {
        from: sq(36),
        to: sq(43),
    };

    let result = EnPassant::try_move(&board, &state, mv);

    assert!(result.is_some());

    let (next, next_state) = result.unwrap();

    assert!(next.pieces[Color::White.index()]
        [Piece::Pawn as usize].contains(sq(43)));

    assert!(!next.pieces[Color::Black.index()]
        [Piece::Pawn as usize].contains(sq(35)));

    assert_eq!(next.side_to_move, Color::Black);
    assert_eq!(next_state.en_passant_target, None);
}

#[test]
fn test_black_en_passant_capture() {
    let (board, state) = black_ep_position();

    let mv = ChessMove {
        from: sq(28),
        to: sq(19),
    };

    let result = EnPassant::try_move(&board, &state, mv);

    assert!(result.is_some());

    let (next, next_state) = result.unwrap();

    assert!(next.pieces[Color::Black.index()]
        [Piece::Pawn as usize].contains(sq(19)));

    assert!(!next.pieces[Color::White.index()]
        [Piece::Pawn as usize].contains(sq(27)));

    assert_eq!(next.side_to_move, Color::White);
    assert_eq!(next_state.en_passant_target, None);
}

#[test]
fn test_en_passant_target_expires_after_next_move() {
    let mut state = GameState::default();

    state.en_passant_target = Some(43);

    state.update_after_move(
        Color::White,
        sq(8),
        sq(16),
        false,
        false,
        false,
    );

    assert_eq!(state.en_passant_target, None);
}

#[test]
fn test_en_passant_rejected_when_target_is_occupied() {
    let (mut board, state) = white_ep_position();

    board.add_piece(Color::White, Piece::Knight, sq(43));

    let mv = ChessMove {
        from: sq(36),
        to: sq(43),
    };

    assert!(EnPassant::try_move(&board, &state, mv).is_none());
}

#[test]
fn test_en_passant_rejected_when_it_exposes_own_king() {
    let mut board = ChessBoard::empty();

    // White King H5, White Pawn G5.
    board.add_piece(Color::White, Piece::King, sq(39));
    board.add_piece(Color::White, Piece::Pawn, sq(38));

    // Black Pawn F5 and Rook A5.
    board.add_piece(Color::Black, Piece::King, sq(60));
    board.add_piece(Color::Black, Piece::Pawn, sq(37));
    board.add_piece(Color::Black, Piece::Rook, sq(32));

    board.side_to_move = Color::White;

    let mut state = GameState::default();
    state.en_passant_target = Some(45); // F6

    let mv = ChessMove {
        from: sq(38),
        to: sq(45),
    };

    assert!(EnPassant::try_move(&board, &state, mv).is_none());
}
