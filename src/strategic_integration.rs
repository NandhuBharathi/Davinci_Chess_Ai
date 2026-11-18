
use tch::{Device, Kind, Tensor};

use crate::neural_network::{
    NetworkOutput,
    EMBEDDING_SIZE,
};

use crate::strategy_discovery::{
    StrategicMemory,
    StrategicExperience,
    DiscoveryConfig,
    DiscoveryResult,
    DiscoveredPattern,
};

#[derive(Debug)]
pub enum IntegrationError {
    InvalidShape(Vec<i64>),
    TensorConversion(tch::TchError),
}

impl From<tch::TchError> for IntegrationError {
    fn from(error: tch::TchError) -> Self {
        Self::TensorConversion(error)
    }
}

pub struct StrategicIntegration {
    memory: StrategicMemory,
}

impl StrategicIntegration {
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            memory: StrategicMemory::new(config),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(DiscoveryConfig::default())
    }

    // Tensor -> Vec<f32>
    fn embedding_to_vec(
        embedding: &Tensor,
    ) -> Result<Vec<f32>, IntegrationError> {

        let shape = embedding.size();

        if shape != [1, EMBEDDING_SIZE] {
            return Err(
                IntegrationError::InvalidShape(shape)
            );
        }

        let cpu_embedding = embedding
            .to_device(Device::Cpu)
            .to_kind(Kind::Float)
            .view([EMBEDDING_SIZE])
            .contiguous();

        let values = Vec::<f32>::try_from(&cpu_embedding)?;

        Ok(values)
    }

    // Observe a neural network embedding.
    pub fn observe_embedding(
        &mut self,
        embedding: &Tensor,
        reward: f32,
    ) -> Result<Option<DiscoveryResult>, IntegrationError> {

        let values = Self::embedding_to_vec(embedding)?;

        let experience = StrategicExperience {
            embedding: values,
            reward,
        };

        Ok(self.memory.observe(experience))
    }

    // Observe directly from Neural Network output.
    pub fn observe_output(
        &mut self,
        output: &NetworkOutput,
        reward: f32,
    ) -> Result<Option<DiscoveryResult>, IntegrationError> {

        self.observe_embedding(
            &output.embedding,
            reward,
        )
    }

    pub fn pattern_count(&self) -> usize {
        self.memory.pattern_count()
    }

    pub fn get_pattern(
        &self,
        id: u64,
    ) -> Option<&DiscoveredPattern> {
        self.memory.get_pattern(id)
    }

    pub fn best_pattern(&self) -> Option<&DiscoveredPattern> {
        self.memory.best_pattern()
    }

    pub fn patterns(&self) -> Vec<&DiscoveredPattern> {
        self.memory.patterns()
    }

    pub fn clear(&mut self) {
        self.memory.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tch::{Device, Kind, Tensor};

    use crate::neural_network::ChessNetwork;

    #[test]
    fn embedding_is_stored_in_memory() {
        let mut integration =
            StrategicIntegration::with_default_config();

        let embedding = Tensor::ones(
            [1, EMBEDDING_SIZE],
            (Kind::Float, Device::Cpu),
        );

        let result = integration
            .observe_embedding(&embedding, 1.0)
            .unwrap();

        assert_eq!(
            result,
            Some(DiscoveryResult::NewPattern(0))
        );

        assert_eq!(integration.pattern_count(), 1);
    }

    #[test]
    fn similar_embedding_updates_existing_pattern() {
        let mut integration =
            StrategicIntegration::with_default_config();

        let first = Tensor::ones(
            [1, EMBEDDING_SIZE],
            (Kind::Float, Device::Cpu),
        );

        let second = Tensor::full(
            [1, EMBEDDING_SIZE],
            1.01,
            (Kind::Float, Device::Cpu),
        );

        integration
            .observe_embedding(&first, 1.0)
            .unwrap();

        let result = integration
            .observe_embedding(&second, 0.5)
            .unwrap();

        assert_eq!(
            result,
            Some(DiscoveryResult::ExistingPattern(0))
        );

        assert_eq!(
            integration.get_pattern(0).unwrap().experience_count,
            2
        );
    }

    #[test]
    fn rejects_wrong_embedding_shape() {
        let mut integration =
            StrategicIntegration::with_default_config();

        let embedding = Tensor::zeros(
            [1, 128],
            (Kind::Float, Device::Cpu),
        );

        let result = integration.observe_embedding(
            &embedding,
            1.0,
        );

        assert!(matches!(
            result,
            Err(IntegrationError::InvalidShape(_))
        ));
    }

    #[test]
    fn rejects_non_finite_embedding() {
        let mut integration =
            StrategicIntegration::with_default_config();

        let embedding = Tensor::full(
            [1, EMBEDDING_SIZE],
            f64::NAN,
            (Kind::Float, Device::Cpu),
        );

        let result = integration.observe_embedding(
            &embedding,
            1.0,
        );

        assert_eq!(result.unwrap(), None);
        assert_eq!(integration.pattern_count(), 0);
    }

    #[test]
    fn integrates_neural_network_output() {
        let network = ChessNetwork::new(Device::Cpu);

        let input = Tensor::zeros(
            [1, crate::board_encoder::ENCODED_SIZE as i64],
            (Kind::Float, Device::Cpu),
        );

        let output = network.forward(&input);

        let mut integration =
            StrategicIntegration::with_default_config();

        let result = integration
            .observe_output(&output, 1.0)
            .unwrap();

        assert!(result.is_some());
        assert_eq!(integration.pattern_count(), 1);
    }
}
