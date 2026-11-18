
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    move_executor::MoveExecutor,
    state_manager::BoardStateHistory,
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn initial_board() -> ChessBoard {
    ChessBoard::initial_position()
}

fn execute(
    board: &ChessBoard,
    state: &GameState,
    from: (u8, u8),
    to: (u8, u8),
) -> (ChessBoard, GameState) {
    MoveExecutor::execute(
        board,
        state,
        ChessMove {
            from: sq(from.0, from.1),
            to: sq(to.0, to.1),
        },
    )
    .expect("Legal move expected")
}

#[test]
fn test_real_e2e4_move() {
    let board = initial_board();
    let state = GameState::new();

    let (next_board, next_state) =
        execute(&board, &state, (4, 1), (4, 3));

    assert_eq!(next_board.side_to_move, Color::Black);
    assert_eq!(next_state.ply, 1);
    assert_eq!(next_state.en_passant_target, Some(20));
}

#[test]
fn test_real_three_move_sequence() {
    let mut board = initial_board();
    let mut state = GameState::new();

    (board, state) = execute(
        &board, &state, (4, 1), (4, 3)
    );

    (board, state) = execute(
        &board, &state, (4, 6), (4, 4)
    );

    (board, state) = execute(
        &board, &state, (6, 0), (5, 2)
    );

    assert_eq!(board.side_to_move, Color::Black);
    assert_eq!(state.ply, 3);
    assert_eq!(state.en_passant_target, None);
}

#[test]
fn test_undo_real_move_sequence() {
    let initial = initial_board();
    let initial_state = GameState::new();

    let mut board = initial.clone();
    let mut state = initial_state.clone();

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    for (from, to, notation) in [
        ((4, 1), (4, 3), "e2e4"),
        ((4, 6), (4, 4), "e7e5"),
        ((6, 0), (5, 2), "g1f3"),
    ] {
        (board, state) = execute(&board, &state, from, to);
        history.save_position(&board, &state, notation.into());
    }

    let restored = history.undo().unwrap();

    assert_eq!(restored.board.side_to_move, Color::White);
    assert_eq!(restored.state.ply, 2);

    assert_eq!(
        restored.board.pieces[Color::White.index()][Piece::Knight as usize].0,
        initial.pieces[Color::White.index()][Piece::Knight as usize].0
    );
}

#[test]
fn test_redo_real_move_sequence() {
    let mut board = initial_board();
    let mut state = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    for (from, to, notation) in [
        ((4, 1), (4, 3), "e2e4"),
        ((4, 6), (4, 4), "e7e5"),
        ((6, 0), (5, 2), "g1f3"),
    ] {
        (board, state) = execute(&board, &state, from, to);
        history.save_position(&board, &state, notation.into());
    }

    history.undo();
    let restored = history.redo().unwrap();

    assert_eq!(restored.board.side_to_move, Color::Black);
    assert_eq!(restored.state.ply, 3);
}

#[test]
fn test_undo_restores_en_passant_state() {
    let mut board = initial_board();
    let mut state = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    (board, state) = execute(
        &board, &state, (4, 1), (4, 3)
    );

    history.save_position(&board, &state, "e2e4".into());

    assert_eq!(state.en_passant_target, Some(20));

    let restored = history.undo().unwrap();

    assert_eq!(restored.state.en_passant_target, None);
    assert_eq!(restored.state.ply, 0);
}

#[test]
fn test_undo_restores_halfmove_clock() {
    let mut board = initial_board();
    let mut state = GameState::new();

    state.halfmove_clock = 7;

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    (board, state) = execute(
        &board, &state, (6, 0), (5, 2)
    );

    history.save_position(&board, &state, "g1f3".into());

    let restored = history.undo().unwrap();

    assert_eq!(restored.state.halfmove_clock, 7);
}

#[test]
fn test_undo_restores_castling_rights() {
    let mut board = initial_board();
    let mut state = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    (board, state) = execute(
        &board, &state, (4, 1), (4, 3)
    );
    history.save_position(&board, &state, "e2e4".into());

    (board, state) = execute(
        &board, &state, (0, 6), (0, 5)
    );
    history.save_position(&board, &state, "a7a6".into());

    (board, state) = execute(
        &board, &state, (4, 0), (4, 1)
    );

    history.save_position(&board, &state, "e1e2".into());

    let restored = history.undo().unwrap();

    assert!(restored.state.castling.white_kingside);
    assert!(restored.state.castling.white_queenside);
}

#[test]
fn test_invalid_move_does_not_change_original() {
    let board = initial_board();
    let state = GameState::new();

    let result = MoveExecutor::execute(
        &board,
        &state,
        ChessMove {
            from: sq(4, 1),
            to: sq(4, 5),
        },
    );

    assert!(result.is_none());
    assert_eq!(board.side_to_move, Color::White);
    assert_eq!(state.ply, 0);
}

#[test]
fn test_new_move_after_undo_creates_new_branch() {
    let mut board = initial_board();
    let mut state = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&board, &state);

    (board, state) = execute(
        &board, &state, (4, 1), (4, 3)
    );
    history.save_position(&board, &state, "e2e4".into());

    let restored = history.undo().unwrap();
    board = restored.board;
    state = restored.state;

    (board, state) = execute(
        &board, &state, (3, 1), (3, 3)
    );
    history.save_position(&board, &state, "d2d4".into());

    assert!(!history.can_redo());
    assert_eq!(history.current().unwrap().state.ply, 1);
}

#[test]
fn test_original_board_remains_unchanged() {
    let original = initial_board();
    let state = GameState::new();

    let original_count = original.total_pieces();

    let (next_board, _) = execute(
        &original, &state, (4, 1), (4, 3)
    );

    assert_eq!(original.total_pieces(), original_count);
    assert_eq!(next_board.total_pieces(), original_count);
}
