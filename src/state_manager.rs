
use crate::{
    ChessBoard,
    game_state::GameState,
    chess_action::{ChessAction, ActionExecutor},
};

#[derive(Debug, Clone)]
pub struct GameSnapshot {
    pub board: ChessBoard,
    pub state: GameState,
    pub move_notation: Option<String>,
}

pub struct BoardStateHistory {
    snapshots: Vec<GameSnapshot>,
    cursor: usize,
}

impl BoardStateHistory {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            cursor: 0,
        }
    }

    pub fn initialize(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
    ) {
        self.snapshots.clear();

        self.snapshots.push(GameSnapshot {
            board: board.clone(),
            state: state.clone(),
            move_notation: None,
        });

        self.cursor = 0;
    }

    pub fn save_position(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
        notation: String,
    ) {
        // Remove future snapshots after a new move.
        self.snapshots.truncate(self.cursor + 1);

        self.snapshots.push(GameSnapshot {
            board: board.clone(),
            state: state.clone(),
            move_notation: Some(notation),
        });

        self.cursor = self.snapshots.len() - 1;
    }

    pub fn undo(&mut self) -> Option<GameSnapshot> {
        if self.cursor == 0 || self.snapshots.is_empty() {
            return None;
        }

        self.cursor -= 1;
        Some(self.snapshots[self.cursor].clone())
    }

    pub fn redo(&mut self) -> Option<GameSnapshot> {
        if self.cursor + 1 >= self.snapshots.len() {
            return None;
        }

        self.cursor += 1;
        Some(self.snapshots[self.cursor].clone())
    }

    pub fn current(&self) -> Option<&GameSnapshot> {
        self.snapshots.get(self.cursor)
    }

    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor + 1 < self.snapshots.len()
    }

    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.cursor = 0;
    }
}

impl Default for BoardStateHistory {
    fn default() -> Self {
        Self::new()
    }
}

// Independent state for hypothetical search branches.
#[derive(Debug, Clone)]
pub struct SearchState {
    pub board: ChessBoard,
    pub game_state: GameState,
    pub depth: u32,
}

impl SearchState {
    pub fn new(
        board: &ChessBoard,
        state: &GameState,
    ) -> Self {
        Self {
            board: board.clone(),
            game_state: state.clone(),
            depth: 0,
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            board: self.board.clone(),
            game_state: self.game_state.clone(),
            depth: self.depth,
        }
    }

    pub fn apply_action(
        &mut self,
        action: &ChessAction,
    ) -> bool {
        match ActionExecutor::execute(
            &self.board,
            &self.game_state,
            action.clone(),
        ) {
            Some((new_board, new_state)) => {
                self.board = new_board;
                self.game_state = new_state;
                self.depth += 1;
                true
            }

            None => false,
        }
    }
}
