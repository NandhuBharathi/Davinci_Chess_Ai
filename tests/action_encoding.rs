
use davinci_chess::{
    ChessBoard, Color, Piece, Square,
    action_encoding::{
        ActionEncoder,
        ACTION_SPACE,
    },
    chess_action::ChessAction,
    move_executor::PromotionChoice,
    move_generator::ChessMove,
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
fn test_action_space_size() {
    assert_eq!(ACTION_SPACE, 4672);
}

#[test]
fn test_normal_move_encode_decode() {
    let board = ChessBoard::initial_position();

    let action = ChessAction::Move(ChessMove {
        from: Square(12),
        to: Square(28),
    });

    let encoded = ActionEncoder::encode(&board, action).unwrap();
    let decoded = ActionEncoder::decode(&board, encoded).unwrap();

    assert_eq!(decoded, action);
}

#[test]
fn test_knight_move_encode_decode() {
    let board = ChessBoard::initial_position();

    let action = ChessAction::Move(ChessMove {
        from: Square(1),
        to: Square(18),
    });

    let encoded = ActionEncoder::encode(&board, action).unwrap();
    let decoded = ActionEncoder::decode(&board, encoded).unwrap();

    assert_eq!(decoded, action);
}

#[test]
fn test_castling_encode_decode() {
    let mut board = ChessBoard::empty();

    board.add_piece(Color::White, Piece::King, Square(4));
    board.add_piece(Color::White, Piece::Rook, Square(7));
    board.add_piece(Color::Black, Piece::King, Square(60));

    board.side_to_move = Color::White;

    let action = ChessAction::Move(ChessMove {
        from: Square(4),
        to: Square(6),
    });

    let encoded = ActionEncoder::encode(&board, action).unwrap();
    let decoded = ActionEncoder::decode(&board, encoded).unwrap();

    assert_eq!(decoded, action);
}

#[test]
fn test_all_promotion_choices_round_trip() {
    let board = promotion_board();

    let choices = [
        PromotionChoice::Queen,
        PromotionChoice::Rook,
        PromotionChoice::Bishop,
        PromotionChoice::Knight,
    ];

    for choice in choices {
        let action = ChessAction::Promote(
            ChessMove {
                from: Square(48),
                to: Square(56),
            },
            choice,
        );

        let encoded = ActionEncoder::encode(&board, action).unwrap();
        let decoded = ActionEncoder::decode(&board, encoded).unwrap();

        assert_eq!(decoded, action);
    }
}

#[test]
fn test_underpromotion_planes_are_unique() {
    let board = promotion_board();

    let choices = [
        PromotionChoice::Rook,
        PromotionChoice::Bishop,
        PromotionChoice::Knight,
    ];

    let mut indices = Vec::new();

    for choice in choices {
        let action = ChessAction::Promote(
            ChessMove {
                from: Square(48),
                to: Square(56),
            },
            choice,
        );

        indices.push(ActionEncoder::encode(&board, action).unwrap());
    }

    indices.sort_unstable();
    indices.dedup();

    assert_eq!(indices.len(), 3);
}

#[test]
fn test_invalid_action_index() {
    let board = ChessBoard::initial_position();

    assert!(ActionEncoder::decode(&board, 4672).is_none());
}

#[test]
fn test_wrong_side_piece_cannot_encode() {
    let board = ChessBoard::initial_position();

    let action = ChessAction::Move(ChessMove {
        from: Square(52),
        to: Square(36),
    });

    assert!(ActionEncoder::encode(&board, action).is_none());
}
