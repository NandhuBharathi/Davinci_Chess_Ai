
use crate::action_encoding::{ActionEncoder, ACTION_SPACE};
use crate::chess_action::ActionExecutor;
use crate::game_state::GameState;
use crate::ChessBoard;
use crate::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalActionMask {
    pub mask: Vec<bool>,
    pub legal_count: usize,
}

impl LegalActionMask {
    pub fn generate(
        board: &ChessBoard,
        state: &GameState,
        color: Color,
    ) -> Self {
        let mut mask = vec![false; ACTION_SPACE];

        if color != board.side_to_move {
            return Self {
                mask,
                legal_count: 0,
            };
        }

        let actions = ActionExecutor::generate(board, state, color);

        for action in actions {
            if let Some(index) = ActionEncoder::encode(board, action) {
                if index < ACTION_SPACE {
                    mask[index] = true;
                }
            }
        }

        let legal_count = mask.iter().filter(|&&x| x).count();

        Self {
            mask,
            legal_count,
        }
    }

    pub fn is_legal(&self, action_index: usize) -> bool {
        self.mask.get(action_index).copied().unwrap_or(false)
    }

    pub fn legal_indices(&self) -> Vec<usize> {
        self.mask
            .iter()
            .enumerate()
            .filter_map(|(i, &legal)| {
                if legal { Some(i) } else { None }
            })
            .collect()
    }

    pub fn apply_to_policy(&self, policy: &[f32]) -> Option<Vec<f32>> {
        if policy.len() != ACTION_SPACE {
            return None;
        }

        let mut masked = vec![0.0; ACTION_SPACE];

        let mut total = 0.0f32;

        for i in 0..ACTION_SPACE {
            if self.mask[i] {
                let value = policy[i].max(0.0);
                masked[i] = value;
                total += value;
            }
        }

        if total <= 0.0 {
            let count = self.legal_count;

            if count == 0 {
                return Some(masked);
            }

            let probability = 1.0 / count as f32;

            for i in 0..ACTION_SPACE {
                if self.mask[i] {
                    masked[i] = probability;
                }
            }

            return Some(masked);
        }

        for value in masked.iter_mut() {
            *value /= total;
        }

        Some(masked)
    }

    pub fn validate(&self) -> bool {
        self.mask.len() == ACTION_SPACE
            && self.mask.iter().filter(|&&x| x).count()
                == self.legal_count
    }
}
