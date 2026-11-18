
use std::collections::HashMap;

use crate::{
    ChessBoard,
    Color,
    Piece,
    game_state::GameState,
    en_passant::EnPassant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub position_hash: u64,
    pub move_notation: Option<String>,
    pub ply: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepetitionStatus {
    None,
    ThreefoldClaimable,
    FivefoldAutomatic,
}

pub struct ZobristHasher;

impl ZobristHasher {
    fn splitmix64(mut value: u64) -> u64 {
        value = value.wrapping_add(0x9E3779B97F4A7C15);
        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58476D1CE4E5B9);
        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D049BB133111EB);
        value ^ (value >> 31)
    }

    fn piece_key(color: usize, piece: usize, square: u8) -> u64 {
        let index =
            (color as u64 * 6 * 64)
            + (piece as u64 * 64)
            + square as u64;

        Self::splitmix64(index + 0x123456789ABCDEF0)
    }

    fn side_key() -> u64 {
        Self::splitmix64(0xABCDEF1234567890)
    }

    fn castling_key(index: u64) -> u64 {
        Self::splitmix64(0xCAFEBABE00000000 + index)
    }

    fn en_passant_key(file: u64) -> u64 {
        Self::splitmix64(0xFACE000000000000 + file)
    }

    pub fn hash(
        board: &ChessBoard,
        state: &GameState,
    ) -> u64 {
        let mut hash = 0u64;

        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                let color_index = color.index();
                let piece_index = piece as usize;

                let mut bits =
                    board.pieces[color_index][piece_index].0;

                while bits != 0 {
                    let square = bits.trailing_zeros() as u8;
                    bits &= bits - 1;

                    hash ^= Self::piece_key(
                        color_index,
                        piece_index,
                        square,
                    );
                }
            }
        }

        if board.side_to_move == Color::Black {
            hash ^= Self::side_key();
        }

        let rights = [
            state.castling.white_kingside,
            state.castling.white_queenside,
            state.castling.black_kingside,
            state.castling.black_queenside,
        ];

        for (index, enabled) in rights.iter().enumerate() {
            if *enabled {
                hash ^= Self::castling_key(index as u64);
            }
        }

        // Include en-passant file only when a legal EP capture exists.
        let legal_ep =
            EnPassant::generate_legal_moves(board, state);

        if !legal_ep.is_empty() {
            if let Some(target) = state.en_passant_target {
                let file = (target % 8) as u64;
                hash ^= Self::en_passant_key(file);
            }
        }

        hash
    }
}

pub struct GameHistory {
    entries: Vec<HistoryEntry>,
    redo_entries: Vec<HistoryEntry>,
    repetition_counts: HashMap<u64, u32>,
}

impl GameHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            redo_entries: Vec::new(),
            repetition_counts: HashMap::new(),
        }
    }

    pub fn start_game(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
    ) -> u64 {
        self.clear();

        let hash = ZobristHasher::hash(board, state);

        self.record_hash(hash, None, state.ply);

        hash
    }

    pub fn record_position(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
        move_notation: String,
    ) -> u64 {
        let hash = ZobristHasher::hash(board, state);

        self.record_hash(
            hash,
            Some(move_notation),
            state.ply,
        );

        hash
    }

    fn record_hash(
        &mut self,
        hash: u64,
        move_notation: Option<String>,
        ply: u64,
    ) {
        let entry = HistoryEntry {
            position_hash: hash,
            move_notation,
            ply,
        };

        self.entries.push(entry);

        *self.repetition_counts.entry(hash).or_insert(0) += 1;

        // A new move invalidates the previous redo branch.
        self.redo_entries.clear();
    }

    pub fn repetition_count(&self, hash: u64) -> u32 {
        *self.repetition_counts.get(&hash).unwrap_or(&0)
    }

    pub fn repetition_status(&self, hash: u64) -> RepetitionStatus {
        match self.repetition_count(hash) {
            5.. => RepetitionStatus::FivefoldAutomatic,
            3..=4 => RepetitionStatus::ThreefoldClaimable,
            _ => RepetitionStatus::None,
        }
    }

    pub fn undo(&mut self) -> Option<HistoryEntry> {
        if self.entries.len() <= 1 {
            return None;
        }

        let entry = self.entries.pop()?;

        if let Some(count) =
            self.repetition_counts.get_mut(&entry.position_hash)
        {
            *count -= 1;

            if *count == 0 {
                self.repetition_counts.remove(&entry.position_hash);
            }
        }

        self.redo_entries.push(entry.clone());

        Some(entry)
    }

    pub fn redo(&mut self) -> Option<HistoryEntry> {
        let entry = self.redo_entries.pop()?;

        *self.repetition_counts
            .entry(entry.position_hash)
            .or_insert(0) += 1;

        self.entries.push(entry.clone());

        Some(entry)
    }

    pub fn current(&self) -> Option<&HistoryEntry> {
        self.entries.last()
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.redo_entries.clear();
        self.repetition_counts.clear();
    }
}

impl Default for GameHistory {
    fn default() -> Self {
        Self::new()
    }
}
