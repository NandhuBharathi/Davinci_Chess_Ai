
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    terminal_state::{
        TerminalState,
    },
    terminal_state_v2::{
        TerminalEvaluatorV2,
        RepetitionTrackerV2,
        RepetitionTrackerV2State,
    },
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn board_with_kings() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));

    board
}

#[test]
fn test_checkmate_detection() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(5, 5));
    board.add_piece(Color::Black, Piece::King, sq(7, 7));
    board.add_piece(Color::White, Piece::Queen, sq(6, 6));

    board.side_to_move = Color::Black;

    assert!(
        TerminalEvaluatorV2::is_checkmate(
            &board,
            &GameState::new()
        )
    );
}

#[test]
fn test_stalemate_detection() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(5, 5));
    board.add_piece(Color::Black, Piece::King, sq(7, 7));
    board.add_piece(Color::White, Piece::Queen, sq(6, 5));

    board.side_to_move = Color::Black;

    assert!(
        TerminalEvaluatorV2::is_stalemate(
            &board,
            &GameState::new()
        )
    );
}

#[test]
fn test_checkmate_winner_black() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(5, 5));
    board.add_piece(Color::Black, Piece::King, sq(7, 7));
    board.add_piece(Color::White, Piece::Queen, sq(6, 6));

    board.side_to_move = Color::Black;

    let result = TerminalEvaluatorV2::evaluate(
        &board,
        &GameState::new(),
        1,
    );

    assert_eq!(
        result,
        TerminalState::Checkmate {
            winner: Color::White
        }
    );
}

#[test]
fn test_king_two_knights_not_insufficient() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Knight, sq(1, 1));
    board.add_piece(Color::White, Piece::Knight, sq(2, 2));

    assert!(
        !TerminalEvaluatorV2::is_insufficient_material(&board)
    );
}

#[test]
fn test_same_color_bishops_insufficient() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Bishop, sq(2, 0));
    board.add_piece(Color::Black, Piece::Bishop, sq(4, 2));

    assert!(
        TerminalEvaluatorV2::is_insufficient_material(&board)
    );
}

#[test]
fn test_opposite_color_bishops_not_insufficient() {
    let mut board = board_with_kings();

    board.add_piece(Color::White, Piece::Bishop, sq(2, 0));
    board.add_piece(Color::Black, Piece::Bishop, sq(3, 2));

    assert!(
        !TerminalEvaluatorV2::is_insufficient_material(&board)
    );
}

#[test]
fn test_repetition_key_is_deterministic() {
    let board = board_with_kings();
    let state = GameState::new();

    let first = RepetitionTrackerV2::position_key(
        &board,
        &state,
    );

    let second = RepetitionTrackerV2::position_key(
        &board,
        &state,
    );

    assert_eq!(first, second);
}

#[test]
fn test_no_en_passant_key() {
    let board = board_with_kings();
    let state = GameState::new();

    let key = RepetitionTrackerV2::position_key(
        &board,
        &state,
    );

    assert!(key.contains("ep:none"));
}

#[test]
fn test_repetition_v2_counts() {
    let board = board_with_kings();
    let state = GameState::new();

    let mut tracker = RepetitionTrackerV2State::new();

    assert_eq!(tracker.record(&board, &state), 1);
    assert_eq!(tracker.record(&board, &state), 2);
    assert_eq!(tracker.record(&board, &state), 3);

    assert_eq!(tracker.count(&board, &state), 3);
}

#[test]
fn test_repetition_key_ignores_unusable_en_passant() {
    let board = board_with_kings();

    let state_without_ep = GameState::new();
    let mut state_with_ep = GameState::new();

    state_with_ep.en_passant_target = Some(20);

    let first = RepetitionTrackerV2::position_key(
        &board,
        &state_without_ep,
    );

    let second = RepetitionTrackerV2::position_key(
        &board,
        &state_with_ep,
    );

    assert_eq!(first, second);
}
