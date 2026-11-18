
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    special_moves::SpecialMoves,
};

fn sq(index: u8) -> Square {
    Square(index)
}

fn black_castling_position() -> (ChessBoard, GameState) {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::Black, Piece::King, sq(60));
    board.add_piece(Color::Black, Piece::Rook, sq(56));
    board.add_piece(Color::Black, Piece::Rook, sq(63));
    board.add_piece(Color::White, Piece::King, sq(4));

    board.side_to_move = Color::Black;

    (board, GameState::default())
}

fn white_castling_position() -> (ChessBoard, GameState) {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4));
    board.add_piece(Color::White, Piece::Rook, sq(0));
    board.add_piece(Color::White, Piece::Rook, sq(7));
    board.add_piece(Color::Black, Piece::King, sq(60));

    board.side_to_move = Color::White;

    (board, GameState::default())
}

#[test]
fn test_black_kingside_castling() {
    let (board, state) = black_castling_position();

    let mv = ChessMove {
        from: sq(60),
        to: sq(62),
    };

    let result = SpecialMoves::try_castle(&board, &state, mv);

    assert!(result.is_some());

    let (new_board, _) = result.unwrap();

    assert!(new_board.pieces[Color::Black.index()][Piece::King as usize]
        .contains(sq(62)));

    assert!(new_board.pieces[Color::Black.index()][Piece::Rook as usize]
        .contains(sq(61)));
}

#[test]
fn test_black_queenside_castling() {
    let (board, state) = black_castling_position();

    let mv = ChessMove {
        from: sq(60),
        to: sq(58),
    };

    let result = SpecialMoves::try_castle(&board, &state, mv);

    assert!(result.is_some());

    let (new_board, _) = result.unwrap();

    assert!(new_board.pieces[Color::Black.index()][Piece::King as usize]
        .contains(sq(58)));

    assert!(new_board.pieces[Color::Black.index()][Piece::Rook as usize]
        .contains(sq(59)));
}

#[test]
fn test_king_cannot_cross_attacked_transit_square() {
    let (mut board, state) = white_castling_position();

    // Black rook A1 attacks F1 after White King vacates E1.
    board.add_piece(Color::Black, Piece::Rook, sq(0));

    // Remove White rook from A1 to avoid occupying the same square.
    board.pieces[Color::White.index()][Piece::Rook as usize].clear(sq(0));
    board.rebuild_occupancy();

    let mv = ChessMove {
        from: sq(4),
        to: sq(6),
    };

    assert!(SpecialMoves::try_castle(&board, &state, mv).is_none());
}

#[test]
fn test_king_cannot_land_on_attacked_destination() {
    let (mut board, state) = white_castling_position();

    // Black rook G8 attacks G1.
    board.add_piece(Color::Black, Piece::Rook, sq(62));

    let mv = ChessMove {
        from: sq(4),
        to: sq(6),
    };

    assert!(SpecialMoves::try_castle(&board, &state, mv).is_none());
}

#[test]
fn test_black_castling_rejected_when_destination_attacked() {
    let (mut board, state) = black_castling_position();

    // White rook G1 attacks G8.
    board.add_piece(Color::White, Piece::Rook, sq(6));

    let mv = ChessMove {
        from: sq(60),
        to: sq(62),
    };

    assert!(SpecialMoves::try_castle(&board, &state, mv).is_none());
}
