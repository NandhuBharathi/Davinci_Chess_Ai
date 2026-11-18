
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    game_state::GameState,
    move_generator::ChessMove,
    move_executor::PromotionChoice,
    chess_action::{ChessAction, ActionExecutor},
};

fn promotion_board() -> ChessBoard {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::Black, Piece::King, Square(60));
    board.add_piece(Color::White, Piece::Pawn, Square(48));

    board.side_to_move = Color::White;
    board
}

#[test]
fn test_promotion_generates_four_actions() {
    let board = promotion_board();
    let state = GameState::default();

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::White,
    );

    let promotions: Vec<_> = actions.iter()
        .filter(|action| matches!(action, ChessAction::Promote(_, _)))
        .copied()
        .collect();

    assert_eq!(promotions.len(), 4);

    assert!(promotions.contains(
        &ChessAction::Promote(
            ChessMove { from: Square(48), to: Square(56) },
            PromotionChoice::Queen
        )
    ));

    assert!(promotions.contains(
        &ChessAction::Promote(
            ChessMove { from: Square(48), to: Square(56) },
            PromotionChoice::Rook
        )
    ));

    assert!(promotions.contains(
        &ChessAction::Promote(
            ChessMove { from: Square(48), to: Square(56) },
            PromotionChoice::Bishop
        )
    ));

    assert!(promotions.contains(
        &ChessAction::Promote(
            ChessMove { from: Square(48), to: Square(56) },
            PromotionChoice::Knight
        )
    ));
}

#[test]
fn test_normal_moves_are_generated() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::White,
    );

    assert!(!actions.is_empty());

    assert!(actions.iter().any(|action| {
        matches!(
            action,
            ChessAction::Move(ChessMove {
                from: Square(12),
                to: Square(28)
            })
        )
    }));
}

#[test]
fn test_promotion_action_execution() {
    let board = promotion_board();
    let state = GameState::default();

    let action = ChessAction::Promote(
        ChessMove {
            from: Square(48),
            to: Square(56),
        },
        PromotionChoice::Queen,
    );

    let (next, _) = ActionExecutor::execute(
        &board,
        &state,
        action,
    ).unwrap();

    assert!(next.pieces[Color::White.index()]
        [Piece::Queen as usize].contains(Square(56)));
}

#[test]
fn test_en_passant_action_generation() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::Black, Piece::King, Square(60));
    board.add_piece(Color::White, Piece::Pawn, Square(36));
    board.add_piece(Color::Black, Piece::Pawn, Square(35));

    board.side_to_move = Color::White;

    let mut state = GameState::default();
    state.en_passant_target = Some(43);

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::White,
    );

    assert!(actions.contains(
        &ChessAction::Move(ChessMove {
            from: Square(36),
            to: Square(43),
        })
    ));
}

#[test]
fn test_castling_action_generation() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::White, Piece::Rook, Square(7));
    board.add_piece(Color::Black, Piece::King, Square(60));

    board.side_to_move = Color::White;

    let state = GameState::default();

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::White,
    );

    assert!(actions.contains(
        &ChessAction::Move(ChessMove {
            from: Square(4),
            to: Square(6),
        })
    ));
}

#[test]
fn test_no_duplicate_actions() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::White,
    );

    for (index, action) in actions.iter().enumerate() {
        assert!(!actions[index + 1..].contains(action));
    }
}

#[test]
fn test_wrong_side_has_no_actions() {
    let board = ChessBoard::initial_position();
    let state = GameState::default();

    let actions = ActionExecutor::generate(
        &board,
        &state,
        Color::Black,
    );

    assert!(actions.is_empty());
}
