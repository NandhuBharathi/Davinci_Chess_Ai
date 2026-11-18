
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    terminal_state::{
        TerminalEvaluator,
        TerminalState,
        GameResult,
        RepetitionTracker,
    },
};

fn sq(file: u8, rank: u8) -> Square {
    Square::from_coords(file, rank).unwrap()
}

fn basic_board() -> ChessBoard {
    let mut board = ChessBoard::empty();
    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));
    board.add_piece(Color::White, Piece::Rook, sq(0, 0));
    board
}

#[test]
fn test_initial_position_ongoing() {
    let board = basic_board();
    let state = GameState::new();

    assert_eq!(
        TerminalEvaluator::evaluate(&board, &state, 1),
        TerminalState::Ongoing
    );
}

#[test]
fn test_fifty_move_claimable() {
    let board = basic_board();
    let mut state = GameState::new();
    state.halfmove_clock = 100;

    assert_eq!(
        TerminalEvaluator::evaluate(&board, &state, 1),
        TerminalState::FiftyMoveClaimable
    );
}

#[test]
fn test_seventy_five_move_draw() {
    let board = basic_board();
    let mut state = GameState::new();
    state.halfmove_clock = 150;

    assert_eq!(
        TerminalEvaluator::evaluate(&board, &state, 1),
        TerminalState::SeventyFiveMoveDraw
    );
}

#[test]
fn test_threefold_claimable() {
    let board = basic_board();
    let state = GameState::new();

    assert_eq!(
        TerminalEvaluator::evaluate(&board, &state, 3),
        TerminalState::ThreefoldRepetitionClaimable
    );
}

#[test]
fn test_fivefold_automatic_draw() {
    let board = basic_board();
    let state = GameState::new();

    assert_eq!(
        TerminalEvaluator::evaluate(&board, &state, 5),
        TerminalState::FivefoldRepetitionDraw
    );
}

#[test]
fn test_king_vs_king_insufficient_material() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));

    assert!(
        TerminalEvaluator::is_insufficient_material(&board)
    );
}

#[test]
fn test_king_bishop_vs_king_insufficient_material() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, sq(4, 0));
    board.add_piece(Color::Black, Piece::King, sq(4, 7));
    board.add_piece(Color::White, Piece::Bishop, sq(2, 2));

    assert!(
        TerminalEvaluator::is_insufficient_material(&board)
    );
}

#[test]
fn test_king_rook_vs_king_sufficient_material() {
    let mut board = basic_board();
    board.add_piece(Color::White, Piece::Rook, sq(0, 0));

    assert!(
        !TerminalEvaluator::is_insufficient_material(&board)
    );
}

#[test]
fn test_repetition_tracker_counts_positions() {
    let board = basic_board();
    let state = GameState::new();

    let mut tracker = RepetitionTracker::new();

    assert_eq!(tracker.record(&board, &state), 1);
    assert_eq!(tracker.record(&board, &state), 2);
    assert_eq!(tracker.record(&board, &state), 3);

    assert_eq!(tracker.count(&board, &state), 3);
}

#[test]
fn test_game_result_draw_mapping() {
    assert_eq!(
        TerminalEvaluator::result(TerminalState::Stalemate),
        GameResult::Draw
    );

    assert_eq!(
        TerminalEvaluator::result(TerminalState::Ongoing),
        GameResult::Ongoing
    );
}
