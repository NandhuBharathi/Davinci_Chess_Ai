
use davinci_chess::legal_action_mask::LegalActionMask;
use davinci_chess::game_state::GameState;
use davinci_chess::{ChessBoard, Color};
use davinci_chess::action_encoding::ACTION_SPACE;

#[test]
fn test_mask_size() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    assert_eq!(mask.mask.len(), ACTION_SPACE);
}

#[test]
fn test_initial_white_legal_moves() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    assert_eq!(mask.legal_count, 20);
}

#[test]
fn test_wrong_side_has_no_actions() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::Black);

    assert_eq!(mask.legal_count, 0);
}

#[test]
fn test_legal_indices_count() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    assert_eq!(mask.legal_indices().len(), 20);
}

#[test]
fn test_illegal_action_rejected() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    assert!(!mask.is_legal(ACTION_SPACE - 1));
}

#[test]
fn test_mask_validation() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    assert!(mask.validate());
}

#[test]
fn test_policy_masking() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    let policy = vec![1.0; ACTION_SPACE];
    let masked = mask.apply_to_policy(&policy).unwrap();

    for i in 0..ACTION_SPACE {
        if mask.mask[i] {
            assert!(masked[i] > 0.0);
        } else {
            assert_eq!(masked[i], 0.0);
        }
    }

    let sum: f32 = masked.iter().sum();
    assert!((sum - 1.0).abs() < 0.0001);
}

#[test]
fn test_zero_policy_fallback() {
    let board = ChessBoard::initial_position();
    let state = GameState::new();

    let mask = LegalActionMask::generate(&board, &state, Color::White);

    let policy = vec![0.0; ACTION_SPACE];
    let masked = mask.apply_to_policy(&policy).unwrap();

    let sum: f32 = masked.iter().sum();

    assert!((sum - 1.0).abs() < 0.0001);
}
