use crate::{ChessBoard, Color, Piece, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
}

pub struct MoveGenerator;

impl MoveGenerator {
    const KNIGHT: [(i8, i8); 8] = [
        (1, 2), (2, 1), (2, -1), (1, -2),
        (-1, -2), (-2, -1), (-2, 1), (-1, 2),
    ];

    const KING: [(i8, i8); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];

    pub fn generate_pseudo_legal(
        board: &ChessBoard,
        from: Square,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        let (color, piece) = match Self::piece_at(board, from) {
            Some(value) => value,
            None => return moves,
        };

        match piece {
            Piece::Pawn => Self::pawn_moves(board, from, color, &mut moves),

            Piece::Knight => Self::jump_moves(
                board, from, color, &Self::KNIGHT, &mut moves
            ),

            Piece::King => Self::jump_moves(
                board, from, color, &Self::KING, &mut moves
            ),

            Piece::Bishop => Self::slide_moves(
                board, from, color,
                &[(1, 1), (1, -1), (-1, 1), (-1, -1)],
                &mut moves
            ),

            Piece::Rook => Self::slide_moves(
                board, from, color,
                &[(1, 0), (-1, 0), (0, 1), (0, -1)],
                &mut moves
            ),

            Piece::Queen => Self::slide_moves(
                board, from, color,
                &[
                    (1, 0), (-1, 0), (0, 1), (0, -1),
                    (1, 1), (1, -1), (-1, 1), (-1, -1),
                ],
                &mut moves
            ),
        }

        moves
    }

    fn pawn_moves(
        board: &ChessBoard,
        from: Square,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let file = from.file() as i8;
        let rank = from.rank() as i8;

        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        let next_rank = rank + direction;

        if let Some(to) = Self::square(file, next_rank) {
            if !Self::occupied(board, to) {
                moves.push(ChessMove { from, to });

                let start_rank = match color {
                    Color::White => 1,
                    Color::Black => 6,
                };

                if rank == start_rank {
                    if let Some(two_step) =
                        Self::square(file, rank + direction * 2)
                    {
                        if !Self::occupied(board, two_step) {
                            moves.push(ChessMove {
                                from,
                                to: two_step,
                            });
                        }
                    }
                }
            }
        }

        for df in [-1, 1] {
            if let Some(to) =
                Self::square(file + df, next_rank)
            {
                if Self::color_at(board, to)
                    .is_some_and(|enemy| enemy != color)
                {
                    moves.push(ChessMove { from, to });
                }
            }
        }
    }

    fn jump_moves(
        board: &ChessBoard,
        from: Square,
        color: Color,
        offsets: &[(i8, i8)],
        moves: &mut Vec<ChessMove>,
    ) {
        let file = from.file() as i8;
        let rank = from.rank() as i8;

        for &(df, dr) in offsets {
            if let Some(to) =
                Self::square(file + df, rank + dr)
            {
                if Self::color_at(board, to) != Some(color) {
                    moves.push(ChessMove { from, to });
                }
            }
        }
    }

    fn slide_moves(
        board: &ChessBoard,
        from: Square,
        color: Color,
        directions: &[(i8, i8)],
        moves: &mut Vec<ChessMove>,
    ) {
        let file = from.file() as i8;
        let rank = from.rank() as i8;

        for &(df, dr) in directions {
            let mut f = file + df;
            let mut r = rank + dr;

            while let Some(to) = Self::square(f, r) {
                match Self::color_at(board, to) {
                    Some(c) if c == color => break,

                    Some(_) => {
                        moves.push(ChessMove { from, to });
                        break;
                    }

                    None => moves.push(ChessMove { from, to }),
                }

                f += df;
                r += dr;
            }
        }
    }

    pub fn is_square_attacked(
        board: &ChessBoard,
        target: Square,
        attacker: Color,
    ) -> bool {
        for index in 0..64 {
            let from = Square(index);

            if Self::color_at(board, from) != Some(attacker) {
                continue;
            }

            let (_, piece) = match Self::piece_at(board, from) {
                Some(value) => value,
                None => continue,
            };

            if Self::attacks_square(
                board, from, piece, attacker, target
            ) {
                return true;
            }
        }

        false
    }

    fn attacks_square(
        board: &ChessBoard,
        from: Square,
        piece: Piece,
        color: Color,
        target: Square,
    ) -> bool {
        let df = target.file() as i8 - from.file() as i8;
        let dr = target.rank() as i8 - from.rank() as i8;

        match piece {
            Piece::Pawn => {
                dr == if color == Color::White { 1 } else { -1 }
                    && df.abs() == 1
            }

            Piece::Knight => {
                (df.abs() == 1 && dr.abs() == 2)
                    || (df.abs() == 2 && dr.abs() == 1)
            }

            Piece::King => {
                df.abs() <= 1 && dr.abs() <= 1
                    && (df != 0 || dr != 0)
            }

            Piece::Bishop => {
                df.abs() == dr.abs()
                    && Self::path_clear(board, from, target)
            }

            Piece::Rook => {
                (df == 0 || dr == 0)
                    && (df != 0 || dr != 0)
                    && Self::path_clear(board, from, target)
            }

            Piece::Queen => {
                (df.abs() == dr.abs() || df == 0 || dr == 0)
                    && (df != 0 || dr != 0)
                    && Self::path_clear(board, from, target)
            }
        }
    }

    fn path_clear(
        board: &ChessBoard,
        from: Square,
        to: Square,
    ) -> bool {
        let df = (to.file() as i8 - from.file() as i8).signum();
        let dr = (to.rank() as i8 - from.rank() as i8).signum();

        let mut f = from.file() as i8 + df;
        let mut r = from.rank() as i8 + dr;

        while f != to.file() as i8 || r != to.rank() as i8 {
            let square = Self::square(f, r).unwrap();

            if Self::occupied(board, square) {
                return false;
            }

            f += df;
            r += dr;
        }

        true
    }

    pub fn piece_at_for_validator(
        board: &ChessBoard,
        square: Square,
    ) -> Option<(Color, Piece)> {
        Self::piece_at(board, square)
    }

    pub fn color_at_for_validator(
        board: &ChessBoard,
        square: Square,
    ) -> Option<Color> {
        Self::color_at(board, square)
    }

    fn piece_at(
        board: &ChessBoard,
        square: Square,
    ) -> Option<(Color, Piece)> {
        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                if board.pieces[color.index()][piece as usize]
                    .contains(square)
                {
                    return Some((color, piece));
                }
            }
        }

        None
    }

    fn color_at(
        board: &ChessBoard,
        square: Square,
    ) -> Option<Color> {
        if board.occupied[Color::White.index()].contains(square) {
            Some(Color::White)
        } else if board.occupied[Color::Black.index()].contains(square) {
            Some(Color::Black)
        } else {
            None
        }
    }

    fn occupied(board: &ChessBoard, square: Square) -> bool {
        board.all_occupied.contains(square)
    }

    fn square(file: i8, rank: i8) -> Option<Square> {
        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            return None;
        }

        Square::from_coords(file as u8, rank as u8)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ChessBoard;

    #[test]
    fn test_initial_pawn_moves() {
        let board = ChessBoard::initial_position();
        let from = Square::new(8).unwrap(); // A2

        let moves = MoveGenerator::generate_pseudo_legal(&board, from);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_initial_knight_moves() {
        let board = ChessBoard::initial_position();
        let from = Square::new(1).unwrap(); // B1

        let moves = MoveGenerator::generate_pseudo_legal(&board, from);

        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_initial_blocked_bishop() {
        let board = ChessBoard::initial_position();
        let from = Square::new(2).unwrap(); // C1

        let moves = MoveGenerator::generate_pseudo_legal(&board, from);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_initial_rook_blocked() {
        let board = ChessBoard::initial_position();
        let from = Square::new(0).unwrap(); // A1

        let moves = MoveGenerator::generate_pseudo_legal(&board, from);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_initial_king_blocked() {
        let board = ChessBoard::initial_position();
        let from = Square::new(4).unwrap(); // E1

        let moves = MoveGenerator::generate_pseudo_legal(&board, from);

        assert_eq!(moves.len(), 0);
    }
}
