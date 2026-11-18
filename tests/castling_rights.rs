
use davinci_chess::{
    Color, Square,
    game_state::GameState,
};

#[test]
fn test_rook_capture_disables_enemy_kingside_right() {
    let mut state = GameState::default();

    state.update_after_move(
        Color::White,
        Square(0),
        Square(63),
        false,
        true,
        false,
    );

    assert!(!state.castling.black_kingside);
}

#[test]
fn test_rook_capture_disables_enemy_queenside_right() {
    let mut state = GameState::default();

    state.update_after_move(
        Color::White,
        Square(0),
        Square(56),
        false,
        true,
        false,
    );

    assert!(!state.castling.black_queenside);
}

#[test]
fn test_non_capture_does_not_disable_enemy_rook_right() {
    let mut state = GameState::default();

    state.update_after_move(
        Color::White,
        Square(8),
        Square(63),
        false,
        false,
        false,
    );

    assert!(state.castling.black_kingside);
}

#[test]
fn test_rook_movement_disables_own_right() {
    let mut state = GameState::default();

    state.update_after_move(
        Color::White,
        Square(7),
        Square(15),
        false,
        false,
        false,
    );

    assert!(!state.castling.white_kingside);
    assert!(state.castling.white_queenside);
}
