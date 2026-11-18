
use tch::{no_grad, Device, Kind, Tensor};

use crate::ChessBoard;
use crate::game_state::GameState;
use crate::board_encoder::BoardEncoder;
use crate::legal_action_mask::LegalActionMask;

use crate::neural_network::{
    ChessNetwork,
    NetworkOutput,
};

use crate::strategic_integration::{
    StrategicIntegration,
    IntegrationError,
};

use crate::strategy_discovery::{
    DiscoveryConfig,
    DiscoveryResult,
};

pub struct BrainOutput {
    pub policy_logits: Tensor,
    pub policy_probabilities: Tensor,
    pub value: f32,
    pub embedding: Tensor,
    pub discovery: Option<DiscoveryResult>,
    pub legal_action_mask: Option<Vec<bool>>,
    pub legal_count: usize,
    pub best_action_index: Option<usize>,
}

pub struct DavinciBrain {
    pub network: ChessNetwork,
    strategic_memory: StrategicIntegration,
}

impl DavinciBrain {

    pub fn new(device: Device) -> Self {
        Self {
            network: ChessNetwork::new(device),
            strategic_memory:
                StrategicIntegration::with_default_config(),
        }
    }

    pub fn cpu() -> Self {
        Self::new(Device::Cpu)
    }

    pub fn with_config(
        device: Device,
        config: DiscoveryConfig,
    ) -> Self {
        Self {
            network: ChessNetwork::new(device),
            strategic_memory:
                StrategicIntegration::new(config),
        }
    }

    // GameState-aware forward pass.
    fn forward_board(
        &self,
        board: &ChessBoard,
        state: &GameState,
    ) -> NetworkOutput {

        let encoded =
            BoardEncoder::encode_with_state(board, state);

        let input = Tensor::from_slice(&encoded)
            .view([1, encoded.len() as i64])
            .to_device(self.network.vs.device());

        no_grad(|| self.network.forward(&input))
    }

    // Default-state compatibility inference.
    pub fn infer(
        &self,
        board: &ChessBoard,
    ) -> BrainOutput {
        self.infer_with_state(board, &GameState::new())
    }

    // Full GameState-aware inference.
    pub fn infer_with_state(
        &self,
        board: &ChessBoard,
        state: &GameState,
    ) -> BrainOutput {

        let output = self.forward_board(board, state);

        self.build_output(output, None, None)
    }

    // Legal inference.
    pub fn infer_legal(
        &self,
        board: &ChessBoard,
        state: &GameState,
    ) -> BrainOutput {

        let output = self.forward_board(board, state);

        let mask = LegalActionMask::generate(
            board,
            state,
            board.side_to_move,
        );

        self.build_output(
            output,
            None,
            Some(mask),
        )
    }

    // Default-state strategic observation.
    pub fn think_and_observe(
        &mut self,
        board: &ChessBoard,
        reward: f32,
    ) -> Result<BrainOutput, IntegrationError> {

        self.think_and_observe_with_state(
            board,
            &GameState::new(),
            reward,
        )
    }

    // GameState-aware strategic observation.
    pub fn think_and_observe_with_state(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
        reward: f32,
    ) -> Result<BrainOutput, IntegrationError> {

        let output = self.forward_board(board, state);

        let discovery = self.strategic_memory
            .observe_output(&output, reward)?;

        Ok(self.build_output(
            output,
            discovery,
            None,
        ))
    }

    // GameState-aware legal inference + memory.
    pub fn think_and_observe_legal(
        &mut self,
        board: &ChessBoard,
        state: &GameState,
        reward: f32,
    ) -> Result<BrainOutput, IntegrationError> {

        let output = self.forward_board(board, state);

        let discovery = self.strategic_memory
            .observe_output(&output, reward)?;

        let mask = LegalActionMask::generate(
            board,
            state,
            board.side_to_move,
        );

        Ok(self.build_output(
            output,
            discovery,
            Some(mask),
        ))
    }

    fn build_output(
        &self,
        output: NetworkOutput,
        discovery: Option<DiscoveryResult>,
        legal_mask: Option<LegalActionMask>,
    ) -> BrainOutput {

        let value =
            output.value.double_value(&[0, 0]) as f32;

        let (probabilities, mask, legal_count, best_action) =
            match legal_mask {

                Some(mask) => {

                    let legal_count = mask.legal_count;

                    if legal_count == 0 {
                        (
                            Tensor::zeros_like(&output.policy),
                            Some(mask.mask),
                            0,
                            None,
                        )
                    } else {

                        let mask_values: Vec<f32> =
                            mask.mask.iter()
                                .map(|&x| if x { 1.0 } else { 0.0 })
                                .collect();

                        let mask_tensor =
                            Tensor::from_slice(&mask_values)
                                .view([1, 4672])
                                .to_device(output.policy.device());

                        let inverse_mask =
                            Tensor::ones_like(&mask_tensor)
                                - &mask_tensor;

                        let masked_logits =
                            &output.policy
                                + inverse_mask * -1.0e9;

                        let probabilities =
                            masked_logits.softmax(-1, Kind::Float);

                        let best =
                            probabilities.argmax(-1, false)
                                .int64_value(&[0]) as usize;

                        (
                            probabilities,
                            Some(mask.mask),
                            legal_count,
                            Some(best),
                        )
                    }
                }

                None => {
                    (
                        output.policy.softmax(-1, Kind::Float),
                        None,
                        0,
                        None,
                    )
                }
            };

        BrainOutput {
            policy_logits: output.policy,
            policy_probabilities: probabilities,
            value,
            embedding: output.embedding,
            discovery,
            legal_action_mask: mask,
            legal_count,
            best_action_index: best_action,
        }
    }

    pub fn pattern_count(&self) -> usize {
        self.strategic_memory.pattern_count()
    }

    pub fn best_pattern_id(&self) -> Option<u64> {
        self.strategic_memory
            .best_pattern()
            .map(|pattern| pattern.id)
    }

    pub fn save_model(
        &self,
        path: &str,
    ) -> Result<(), tch::TchError> {
        self.network.save(path)
    }

    pub fn load_model(
        &mut self,
        path: &str,
    ) -> Result<(), tch::TchError> {
        self.network.load(path)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ChessBoard;

    #[test]
    fn brain_inference_output_shapes() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();

        let output = brain.infer(&board);

        assert_eq!(output.policy_logits.size(), vec![1, 4672]);
        assert_eq!(output.policy_probabilities.size(), vec![1, 4672]);
        assert_eq!(output.embedding.size(), vec![1, 256]);

        assert!(output.value >= -1.0);
        assert!(output.value <= 1.0);
    }

    #[test]
    fn state_aware_inference_works() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();
        let state = GameState::new();

        let output = brain.infer_with_state(&board, &state);

        assert_eq!(output.policy_logits.size(), vec![1, 4672]);
    }

    #[test]
    fn policy_probabilities_sum_to_one() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();

        let output = brain.infer(&board);

        let sum = output.policy_probabilities
            .sum(Kind::Float)
            .double_value(&[]);

        assert!((sum - 1.0).abs() < 0.0001);
    }

    #[test]
    fn legal_policy_contains_only_legal_moves() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();
        let state = GameState::new();

        let output = brain.infer_legal(&board, &state);

        let mask = output.legal_action_mask.as_ref().unwrap();

        assert_eq!(output.legal_count, 20);
        assert_eq!(mask.len(), 4672);

        for i in 0..4672 {
            let probability =
                output.policy_probabilities
                    .double_value(&[0, i as i64]);

            if !mask[i] {
                assert_eq!(probability, 0.0);
            }
        }

        let best = output.best_action_index.unwrap();
        assert!(mask[best]);
    }

    #[test]
    fn legal_probabilities_sum_to_one() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();
        let state = GameState::new();

        let output = brain.infer_legal(&board, &state);

        let sum = output.policy_probabilities
            .sum(Kind::Float)
            .double_value(&[]);

        assert!((sum - 1.0).abs() < 0.0001);
    }

    #[test]
    fn think_and_observe_creates_pattern() {
        let mut brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();

        let output = brain
            .think_and_observe(&board, 1.0)
            .unwrap();

        assert_eq!(
            output.discovery,
            Some(DiscoveryResult::NewPattern(0))
        );

        assert_eq!(brain.pattern_count(), 1);
    }

    #[test]
    fn inference_does_not_change_memory() {
        let brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();

        brain.infer(&board);

        assert_eq!(brain.pattern_count(), 0);
    }

    #[test]
    fn repeated_position_updates_memory() {
        let mut brain = DavinciBrain::cpu();
        let board = ChessBoard::initial_position();

        brain.think_and_observe(&board, 1.0).unwrap();

        let output = brain
            .think_and_observe(&board, 0.5)
            .unwrap();

        assert_eq!(
            output.discovery,
            Some(DiscoveryResult::ExistingPattern(0))
        );

        assert_eq!(brain.pattern_count(), 1);
    }
}
