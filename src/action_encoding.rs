
use crate::{
    ChessBoard, Color, Piece, Square,
    chess_action::ChessAction,
    move_executor::PromotionChoice,
    move_generator::ChessMove,
};

pub const BOARD_SQUARES: usize = 64;
pub const ACTION_PLANES: usize = 73;
pub const ACTION_SPACE: usize = BOARD_SQUARES * ACTION_PLANES;

pub struct ActionEncoder;

// Directions: N, NE, E, SE, S, SW, W, NW
const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

// Knight directions.
const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

impl ActionEncoder {

    fn promotion_choice(index: usize) -> Option<PromotionChoice> {
        match index {
            0 => Some(PromotionChoice::Knight),
            1 => Some(PromotionChoice::Bishop),
            2 => Some(PromotionChoice::Rook),
            3 => Some(PromotionChoice::Queen),
            _ => None,
        }
    }

    fn is_promotion_square(color: Color, to: Square) -> bool {
        match color {
            Color::White => to.rank() == 7,
            Color::Black => to.rank() == 0,
        }
    }

    fn queen_plane(dx: i8, dy: i8) -> Option<usize> {
        for (direction_index, (dir_x, dir_y))
            in QUEEN_DIRECTIONS.iter().enumerate()
        {
            for distance in 1..=7 {
                if dx == dir_x * distance
                    && dy == dir_y * distance
                {
                    return Some(direction_index * 7 + (distance - 1) as usize);
                }
            }
        }

        None
    }

    fn knight_plane(dx: i8, dy: i8) -> Option<usize> {
        KNIGHT_DIRECTIONS
            .iter()
            .position(|&(x, y)| x == dx && y == dy)
            .map(|index| 56 + index)
    }

    fn underpromotion_plane(
        color: Color,
        dx: i8,
        dy: i8,
        choice: PromotionChoice,
    ) -> Option<usize> {

        let directions: [(i8, i8); 3] = match color {
            Color::White => [
                (0, 1),   // Forward
                (-1, 1),  // Forward-left
                (1, 1),   // Forward-right
            ],
            Color::Black => [
                (0, -1),
                (-1, -1),
                (1, -1),
            ],
        };

        let direction_index = directions
            .iter()
            .position(|&(x, y)| x == dx && y == dy)?;

        let piece_index = match choice {
            PromotionChoice::Knight => 0,
            PromotionChoice::Bishop => 1,
            PromotionChoice::Rook => 2,
            PromotionChoice::Queen => return None,
        };

        Some(64 + direction_index * 3 + piece_index)
    }

    // ==========================================
    // ENCODE
    // ==========================================

    pub fn encode(
        board: &ChessBoard,
        action: ChessAction,
    ) -> Option<usize> {

        let (chess_move, promotion) = match action {
            ChessAction::Move(mv) => (mv, None),
            ChessAction::Promote(mv, choice) => (mv, Some(choice)),
        };

        let from = chess_move.from;
        let to = chess_move.to;

        if from.0 >= 64 || to.0 >= 64 || from == to {
            return None;
        }

        let color = board.side_to_move;

        let (piece_color, piece) =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, from)?;

        if piece_color != color {
            return None;
        }

        let dx = to.file() as i8 - from.file() as i8;
        let dy = to.rank() as i8 - from.rank() as i8;

        let is_promotion =
            Self::is_promotion_square(color, to)
            && piece == Piece::Pawn;

        let plane = if is_promotion {

            let choice = promotion?;

            if let Some(plane) = Self::underpromotion_plane(
                color,
                dx,
                dy,
                choice,
            ) {
                plane
            } else if choice == PromotionChoice::Queen {
                Self::queen_plane(dx, dy)?
            } else {
                return None;
            }

        } else {

            if promotion.is_some() {
                return None;
            }

            if piece == Piece::Pawn
                && Self::is_promotion_square(color, to)
            {
                return None;
            }

            if let Some(plane) = Self::queen_plane(dx, dy) {
                plane
            } else {
                Self::knight_plane(dx, dy)?
            }
        };

        Some(from.0 as usize * ACTION_PLANES + plane)
    }

    // ==========================================
    // DECODE
    // ==========================================

    pub fn decode(
        board: &ChessBoard,
        action_index: usize,
    ) -> Option<ChessAction> {

        if action_index >= ACTION_SPACE {
            return None;
        }

        let from_index = action_index / ACTION_PLANES;
        let plane = action_index % ACTION_PLANES;

        let from = Square::new(from_index as u8)?;

        let color = board.side_to_move;

        let (piece_color, piece) =
            crate::move_generator::MoveGenerator::
                piece_at_for_validator(board, from)?;

        if piece_color != color {
            return None;
        }

        let (dx, dy) = if plane < 56 {

            let direction = plane / 7;
            let distance = (plane % 7 + 1) as i8;

            let (x, y) = QUEEN_DIRECTIONS[direction];

            (x * distance, y * distance)

        } else if plane < 64 {

            KNIGHT_DIRECTIONS[plane - 56]

        } else {

            if piece != Piece::Pawn {
                return None;
            }

            let promotion_plane = plane - 64;
            let direction = promotion_plane / 3;

            let directions: [(i8, i8); 3] = match color {
                Color::White => [
                    (0, 1),
                    (-1, 1),
                    (1, 1),
                ],
                Color::Black => [
                    (0, -1),
                    (-1, -1),
                    (1, -1),
                ],
            };

            directions[direction]
        };

        let target_file = from.file() as i8 + dx;
        let target_rank = from.rank() as i8 + dy;

        if !(0..8).contains(&target_file)
            || !(0..8).contains(&target_rank)
        {
            return None;
        }

        let to = Square::from_coords(
            target_file as u8,
            target_rank as u8,
        )?;

        let chess_move = ChessMove { from, to };

        // Promotion handling.
        if piece == Piece::Pawn
            && Self::is_promotion_square(color, to)
        {
            if plane >= 64 {
                let choice = Self::promotion_choice(
                    (plane - 64) % 3
                )?;

                return Some(ChessAction::Promote(
                    chess_move,
                    choice,
                ));
            }

            if plane < 56 {
                return Some(ChessAction::Promote(
                    chess_move,
                    PromotionChoice::Queen,
                ));
            }

            return None;
        }

        // Underpromotion planes are only valid for promotion.
        if plane >= 64 {
            return None;
        }

        Some(ChessAction::Move(chess_move))
    }
}
