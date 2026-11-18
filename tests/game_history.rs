
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    game_history::{
        GameHistory,
        ZobristHasher,
        RepetitionStatus,
    },
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
fn test_zobrist_hash_consistency() {
    let b = board();
    let s = GameState::new();

    assert_eq!(
        ZobristHasher::hash(&b, &s),
        ZobristHasher::hash(&b, &s)
    );
}

#[test]
fn test_different_positions_different_hash() {
    let b1 = board();
    let mut b2 = board();

    b2.add_piece(Color::White, Piece::Knight, sq(1, 0));

    assert_ne!(
        ZobristHasher::hash(&b1, &GameState::new()),
        ZobristHasher::hash(&b2, &GameState::new())
    );
}

#[test]
fn test_side_to_move_changes_hash() {
    let b1 = board();
    let mut b2 = board();

    b2.side_to_move = Color::Black;

    assert_ne!(
        ZobristHasher::hash(&b1, &GameState::new()),
        ZobristHasher::hash(&b2, &GameState::new())
    );
}

#[test]
fn test_castling_rights_change_hash() {
    let b = board();

    let s1 = GameState::new();
    let mut s2 = GameState::new();

    s2.castling.white_kingside = false;

    assert_ne!(
        ZobristHasher::hash(&b, &s1),
        ZobristHasher::hash(&b, &s2)
    );
}

#[test]
fn test_initial_position_recording() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();

    history.start_game(&b, &s);

    assert_eq!(history.len(), 1);
    assert!(!history.is_empty());
}

#[test]
fn test_repetition_counting() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();

    let hash = history.start_game(&b, &s);

    history.record_position(&b, &s, "test1".into());

    assert_eq!(history.repetition_count(hash), 2);
}

#[test]
fn test_threefold_detection() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();
    let hash = history.start_game(&b, &s);

    history.record_position(&b, &s, "move1".into());
    history.record_position(&b, &s, "move2".into());

    assert_eq!(
        history.repetition_status(hash),
        RepetitionStatus::ThreefoldClaimable
    );
}

#[test]
fn test_fivefold_detection() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();
    let hash = history.start_game(&b, &s);

    for i in 0..4 {
        history.record_position(&b, &s, format!("move{}", i));
    }

    assert_eq!(
        history.repetition_status(hash),
        RepetitionStatus::FivefoldAutomatic
    );
}

#[test]
fn test_undo_updates_repetition_count() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();
    let hash = history.start_game(&b, &s);

    history.record_position(&b, &s, "move1".into());

    assert_eq!(history.repetition_count(hash), 2);

    history.undo();

    assert_eq!(history.repetition_count(hash), 1);
}

#[test]
fn test_redo_restores_repetition_count() {
    let b = board();
    let s = GameState::new();

    let mut history = GameHistory::new();
    let hash = history.start_game(&b, &s);

    history.record_position(&b, &s, "move1".into());
    history.undo();
    history.redo();

    assert_eq!(history.repetition_count(hash), 2);
}
