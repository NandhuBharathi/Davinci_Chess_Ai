
use tch::{
    nn,
    nn::Module,
    Device,
    Tensor,
};

use crate::action_encoding::ACTION_SPACE;
use crate::board_encoder::ENCODED_SIZE;

pub const HIDDEN_SIZE: i64 = 512;
pub const EMBEDDING_SIZE: i64 = 256;

pub struct NetworkOutput {
    pub policy: Tensor,
    pub value: Tensor,
    pub embedding: Tensor,
}

pub struct ChessNetwork {
    pub vs: nn::VarStore,

    trunk: nn::Sequential,

    policy_head: nn::Linear,
    value_head: nn::Linear,
    embedding_head: nn::Linear,
}

impl ChessNetwork {

    pub fn new(device: Device) -> Self {

        let vs = nn::VarStore::new(device);
        let root = &vs.root();

        let trunk = nn::seq()
            .add(nn::linear(
                root / "trunk_1",
                ENCODED_SIZE as i64,
                HIDDEN_SIZE,
                Default::default(),
            ))
            .add_fn(|x| x.relu())
            .add(nn::linear(
                root / "trunk_2",
                HIDDEN_SIZE,
                HIDDEN_SIZE,
                Default::default(),
            ))
            .add_fn(|x| x.relu());

        let policy_head = nn::linear(
            root / "policy_head",
            HIDDEN_SIZE,
            ACTION_SPACE as i64,
            Default::default(),
        );

        let value_head = nn::linear(
            root / "value_head",
            HIDDEN_SIZE,
            1,
            Default::default(),
        );

        let embedding_head = nn::linear(
            root / "embedding_head",
            HIDDEN_SIZE,
            EMBEDDING_SIZE,
            Default::default(),
        );

        Self {
            vs,
            trunk,
            policy_head,
            value_head,
            embedding_head,
        }
    }

    pub fn forward(&self, input: &Tensor) -> NetworkOutput {

        let hidden = self.trunk.forward(input);

        let policy = self.policy_head.forward(&hidden);

        let value = self
            .value_head
            .forward(&hidden)
            .tanh();

        let embedding =
            self.embedding_head.forward(&hidden);

        NetworkOutput {
            policy,
            value,
            embedding,
        }
    }

    pub fn save(
        &self,
        path: &str,
    ) -> Result<(), tch::TchError> {
        self.vs.save(path)
    }

    pub fn load(
        &mut self,
        path: &str,
    ) -> Result<(), tch::TchError> {
        self.vs.load(path)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn network_output_dimensions() {

        let network = ChessNetwork::new(Device::Cpu);

        let input = Tensor::zeros(
            [1, ENCODED_SIZE as i64],
            (tch::Kind::Float, Device::Cpu),
        );

        let output = network.forward(&input);

        assert_eq!(
            output.policy.size(),
            vec![1, ACTION_SPACE as i64]
        );

        assert_eq!(
            output.value.size(),
            vec![1, 1]
        );

        assert_eq!(
            output.embedding.size(),
            vec![1, EMBEDDING_SIZE]
        );
    }

    #[test]
    fn network_accepts_1280_features() {

        let network = ChessNetwork::new(Device::Cpu);

        let input = Tensor::zeros(
            [1, 1280],
            (tch::Kind::Float, Device::Cpu),
        );

        let output = network.forward(&input);

        assert_eq!(output.policy.size()[0], 1);
    }


    #[test]
    fn model_save_load_preserves_outputs() {
        let network = ChessNetwork::new(Device::Cpu);

        let input = Tensor::randn(
            [1, ENCODED_SIZE as i64],
            (tch::Kind::Float, Device::Cpu),
        );

        let before = network.forward(&input);

        let path = std::env::temp_dir().join(
            format!("davinci_model_{}.ot", std::process::id())
        );

        let path_str = path.to_str().unwrap();

        network.save(path_str).unwrap();

        let mut restored = ChessNetwork::new(Device::Cpu);
        restored.load(path_str).unwrap();

        let after = restored.forward(&input);

        let policy_diff = (&before.policy - &after.policy)
            .abs()
            .max()
            .double_value(&[]);

        let value_diff = (&before.value - &after.value)
            .abs()
            .max()
            .double_value(&[]);

        let embedding_diff = (&before.embedding - &after.embedding)
            .abs()
            .max()
            .double_value(&[]);

        assert!(policy_diff < 1e-6);
        assert!(value_diff < 1e-6);
        assert!(embedding_diff < 1e-6);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn model_file_exists_after_save() {
        let network = ChessNetwork::new(Device::Cpu);

        let path = std::env::temp_dir().join(
            format!("davinci_exists_{}.ot", std::process::id())
        );

        let path_str = path.to_str().unwrap();

        network.save(path_str).unwrap();

        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
    }

}
