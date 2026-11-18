
use crate::{ChessBoard, Color, Piece, Square};

#[derive(Debug, Clone, Copy, Default)]
pub struct PawnStructureBreakdown {
    pub doubled: f32,
    pub isolated: f32,
    pub backward: f32,
    pub islands: f32,
    pub connected: f32,
    pub protected: f32,
    pub passed: f32,
    pub protected_passed: f32,
    pub advancement: f32,
    pub king_distance: f32,
    pub total: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PawnStructureEvaluation {
    pub white: PawnStructureBreakdown,
    pub black: PawnStructureBreakdown,
    pub total: f32,
}

pub struct PawnStructure;

impl PawnStructure {
    const DOUBLED_PENALTY: f32 = 0.50;
    const ISOLATED_PENALTY: f32 = 0.60;
    const BACKWARD_PENALTY: f32 = 0.45;
    const ISLAND_PENALTY: f32 = 0.30;

    const CONNECTED_BONUS: f32 = 0.20;
    const PROTECTED_BONUS: f32 = 0.20;

    const PASSED_BASE: f32 = 0.50;
    const PASSED_ADVANCEMENT: f32 = 0.25;
    const PROTECTED_PASSED_BONUS: f32 = 0.40;

    const BLOCKED_PATH_PENALTY: f32 = 0.30;
    const KING_DISTANCE_WEIGHT: f32 = 0.04;

    pub fn evaluate(board: &ChessBoard) -> PawnStructureEvaluation {
        let white = Self::evaluate_side(board, Color::White);
        let black = Self::evaluate_side(board, Color::Black);

        PawnStructureEvaluation {
            total: white.total - black.total,
            white,
            black,
        }
    }

    fn evaluate_side(
        board: &ChessBoard,
        color: Color,
    ) -> PawnStructureBreakdown {
        let pawns = Self::get_pawns(board, color);
        let enemy = Self::opponent(color);
        let enemy_pawns = Self::get_pawns(board, enemy);

        let mut result = PawnStructureBreakdown::default();

        if pawns.is_empty() {
            return result;
        }

        // 1. Doubled pawns
        let mut file_counts = [0usize; 8];

        for pawn in &pawns {
            file_counts[pawn.file() as usize] += 1;
        }

        for count in file_counts {
            if count > 1 {
                result.doubled -=
                    (count - 1) as f32 * Self::DOUBLED_PENALTY;
            }
        }

        // 2. Isolated pawns
        for pawn in &pawns {
            let file = pawn.file() as i8;

            let has_adjacent_file_pawn = pawns.iter().any(|other| {
                (other.file() as i8 - file).abs() == 1
            });

            if !has_adjacent_file_pawn {
                result.isolated -= Self::ISOLATED_PENALTY;
            }
        }

        // 3. Pawn islands
        let occupied_files: Vec<bool> =
            file_counts.iter().map(|&count| count > 0).collect();

        let mut islands = 0usize;
        let mut previous = false;

        for occupied in occupied_files {
            if occupied && !previous {
                islands += 1;
            }
            previous = occupied;
        }

        if islands > 1 {
            result.islands -=
                (islands - 1) as f32 * Self::ISLAND_PENALTY;
        }

        // 4. Connected pawns and protected pawns
        for i in 0..pawns.len() {
            let pawn = pawns[i];

            if Self::is_protected(pawn, &pawns, color) {
                result.protected += Self::PROTECTED_BONUS;
            }

            for j in (i + 1)..pawns.len() {
                let other = pawns[j];

                let file_diff =
                    (pawn.file() as i8 - other.file() as i8).abs();

                let rank_diff =
                    (pawn.rank() as i8 - other.rank() as i8).abs();

                if file_diff == 1 && rank_diff <= 1 {
                    result.connected += Self::CONNECTED_BONUS;
                }
            }
        }

        // 5. Passed pawns and advancement
        for pawn in &pawns {
            if Self::is_passed(*pawn, &enemy_pawns, color) {
                let advancement = Self::advancement(*pawn, color);

                result.passed += Self::PASSED_BASE;

                result.advancement +=
                    advancement as f32 * Self::PASSED_ADVANCEMENT;

                if Self::is_protected(*pawn, &pawns, color) {
                    result.protected_passed +=
                        Self::PROTECTED_PASSED_BONUS;
                }

                if Self::path_blocked(board, *pawn, color) {
                    result.passed -= Self::BLOCKED_PATH_PENALTY;
                }

                result.king_distance +=
                    Self::king_support_score(board, *pawn, color);
            }
        }

        // 6. Backward pawns
        for pawn in &pawns {
            if Self::is_backward(
                board,
                *pawn,
                &pawns,
                &enemy_pawns,
                color,
            ) {
                result.backward -= Self::BACKWARD_PENALTY;
            }
        }

        result.total =
            result.doubled
            + result.isolated
            + result.backward
            + result.islands
            + result.connected
            + result.protected
            + result.passed
            + result.protected_passed
            + result.advancement
            + result.king_distance;

        result
    }

    fn get_pawns(board: &ChessBoard, color: Color) -> Vec<Square> {
        let index = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::Pawn)
            .unwrap();

        let mut bits = board.pieces[color.index()][index];
        let mut result = Vec::new();

        while let Some(square) = bits.pop_lsb() {
            result.push(square);
        }

        result
    }

    fn is_protected(
        pawn: Square,
        friendly: &[Square],
        color: Color,
    ) -> bool {
        friendly.iter().any(|other| {
            if other == &pawn {
                return false;
            }

            let dx = (pawn.file() as i8 - other.file() as i8).abs();

            let dy = match color {
                Color::White => pawn.rank() as i8 - other.rank() as i8,
                Color::Black => other.rank() as i8 - pawn.rank() as i8,
            };

            dx == 1 && dy == 1
        })
    }

    fn is_passed(
        pawn: Square,
        enemy_pawns: &[Square],
        color: Color,
    ) -> bool {
        let file = pawn.file() as i8;
        let rank = pawn.rank() as i8;

        !enemy_pawns.iter().any(|enemy| {
            let enemy_file = enemy.file() as i8;
            let enemy_rank = enemy.rank() as i8;

            let same_or_adjacent =
                (enemy_file - file).abs() <= 1;

            let ahead = match color {
                Color::White => enemy_rank > rank,
                Color::Black => enemy_rank < rank,
            };

            same_or_adjacent && ahead
        })
    }


    fn is_backward(
        board: &ChessBoard,
        pawn: Square,
        friendly: &[Square],
        enemy_pawns: &[Square],
        color: Color,
    ) -> bool {
        let rank = pawn.rank() as i8;

        let forward = match color {
            Color::White => 1i8,
            Color::Black => -1i8,
        };

        let next_rank = rank + forward;

        if !(0..8).contains(&next_rank) {
            return false;
        }

        let next = Square::from_coords(
            pawn.file(),
            next_rank as u8,
        ).unwrap();

        if board.all_occupied.contains(next) {
            return false;
        }

        // Enemy pawn attacks the advance square
        let enemy_controls_next = enemy_pawns.iter().any(|enemy| {
            let file_diff =
                (enemy.file() as i8 - next.file() as i8).abs();

            let enemy_rank = enemy.rank() as i8;

            let attacks = match color {
                Color::White => enemy_rank == next_rank + 1,
                Color::Black => enemy_rank == next_rank - 1,
            };

            file_diff == 1 && attacks
        });

        if !enemy_controls_next {
            return false;
        }

        // A friendly pawn on the same rank and adjacent file
        // can protect the advance square.
        let has_support = friendly.iter().any(|other| {
            other != &pawn
                && (other.file() as i8 - pawn.file() as i8).abs() == 1
                && other.rank() == pawn.rank()
        });

        !has_support
    }

    fn path_blocked(
        board: &ChessBoard,
        pawn: Square,
        color: Color,
    ) -> bool {
        let direction = match color {
            Color::White => 1i8,
            Color::Black => -1i8,
        };

        let mut rank = pawn.rank() as i8 + direction;

        while (0..8).contains(&rank) {
            let square = Square::from_coords(
                pawn.file(),
                rank as u8,
            ).unwrap();

            if board.all_occupied.contains(square) {
                return true;
            }

            rank += direction;
        }

        false
    }


    fn king_support_score(
        board: &ChessBoard,
        pawn: Square,
        color: Color,
    ) -> f32 {
        let friendly_king = Self::find_king(board, color);
        let enemy_king = Self::find_king(
            board,
            Self::opponent(color),
        );

        let (Some(friendly_king), Some(enemy_king)) =
            (friendly_king, enemy_king)
        else {
            return 0.0;
        };

        let friendly_distance = Self::king_distance(
            friendly_king,
            pawn,
        );

        let enemy_distance = Self::king_distance(
            enemy_king,
            pawn,
        );

        (enemy_distance as f32 - friendly_distance as f32)
            * Self::KING_DISTANCE_WEIGHT
    }

    fn find_king(
        board: &ChessBoard,
        color: Color,
    ) -> Option<Square> {
        let index = Piece::ALL
            .iter()
            .position(|piece| *piece == Piece::King)?;

        let mut bits = board.pieces[color.index()][index];
        bits.pop_lsb()
    }

    fn king_distance(a: Square, b: Square) -> u8 {
        let file_distance =
            (a.file() as i8 - b.file() as i8).abs();

        let rank_distance =
            (a.rank() as i8 - b.rank() as i8).abs();

        file_distance.max(rank_distance) as u8
    }

    fn advancement(pawn: Square, color: Color) -> u8 {
        match color {
            Color::White => pawn.rank(),
            Color::Black => 7 - pawn.rank(),
        }
    }

    fn opponent(color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
