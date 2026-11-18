
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    state_manager::{BoardStateHistory, SearchState},
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn board() -> ChessBoard {
    let mut b = ChessBoard::empty();
    b.add_piece(Color::White, Piece::King, sq(4, 0));
    b.add_piece(Color::Black, Piece::King, sq(4, 7));
    b.add_piece(Color::White, Piece::Rook, sq(0, 0));
    b
}

#[test]
fn test_initial_snapshot() {
    let b = board();
    let s = GameState::new();
    let mut history = BoardStateHistory::new();

    history.initialize(&b, &s);

    assert_eq!(history.len(), 1);
    assert!(history.current().is_some());
}

#[test]
fn test_save_new_position() {
    let b1 = board();
    let mut b2 = board();

    b2.add_piece(Color::White, Piece::Knight, sq(1, 0));

    let s = GameState::new();
    let mut history = BoardStateHistory::new();

    history.initialize(&b1, &s);
    history.save_position(&b2, &s, "b1c3".into());

    assert_eq!(history.len(), 2);
}

#[test]
fn test_undo_restores_previous_board() {
    let b1 = board();
    let mut b2 = board();

    b2.add_piece(Color::White, Piece::Knight, sq(1, 0));

    let s = GameState::new();
    let mut history = BoardStateHistory::new();

    history.initialize(&b1, &s);
    history.save_position(&b2, &s, "b1c3".into());

    let previous = history.undo().unwrap();

    assert_eq!(
        previous.board.total_pieces(),
        b1.total_pieces()
    );
}

#[test]
fn test_redo_restores_next_board() {
    let b1 = board();
    let mut b2 = board();

    b2.add_piece(Color::White, Piece::Knight, sq(1, 0));

    let s = GameState::new();
    let mut history = BoardStateHistory::new();

    history.initialize(&b1, &s);
    history.save_position(&b2, &s, "b1c3".into());

    history.undo();

    let next = history.redo().unwrap();

    assert_eq!(
        next.board.total_pieces(),
        b2.total_pieces()
    );
}

#[test]
fn test_undo_boundary() {
    let b = board();
    let s = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&b, &s);

    assert!(history.undo().is_none());
}

#[test]
fn test_redo_boundary() {
    let b = board();
    let s = GameState::new();

    let mut history = BoardStateHistory::new();
    history.initialize(&b, &s);

    assert!(history.redo().is_none());
}

#[test]
fn test_new_move_clears_redo_branch() {
    let b1 = board();
    let mut b2 = board();
    let mut b3 = board();

    b2.add_piece(Color::White, Piece::Knight, sq(1, 0));
    b3.add_piece(Color::White, Piece::Bishop, sq(2, 0));

    let s = GameState::new();

    let mut history = BoardStateHistory::new();

    history.initialize(&b1, &s);
    history.save_position(&b2, &s, "move1".into());

    history.undo();

    history.save_position(&b3, &s, "move2".into());

    assert!(!history.can_redo());
}

#[test]
fn test_search_state_fork_independent() {
    let b = board();
    let s = GameState::new();

    let original = SearchState::new(&b, &s);
    let forked = original.fork();

    assert_eq!(
        original.board.total_pieces(),
        forked.board.total_pieces()
    );
}

#[test]
fn test_search_branch_does_not_modify_original() {
    let b = board();
    let s = GameState::new();

    let original = SearchState::new(&b, &s);
    let forked = original.fork();

    assert_eq!(
        original.board.total_pieces(),
        b.total_pieces()
    );

    assert_eq!(
        forked.board.total_pieces(),
        b.total_pieces()
    );
}

#[test]
fn test_game_state_restoration() {
    let b = board();
    let s1 = GameState::new();
    let mut s2 = GameState::new();

    s2.halfmove_clock = 25;
    s2.ply = 25;

    let mut history = BoardStateHistory::new();

    history.initialize(&b, &s1);
    history.save_position(&b, &s2, "test".into());

    let restored = history.undo().unwrap();

    assert_eq!(restored.state.halfmove_clock, 0);
    assert_eq!(restored.state.ply, 0);
}
