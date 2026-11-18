
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StrategicExperience {
    pub embedding: Vec<f32>,
    pub reward: f32,
}

#[derive(Debug, Clone)]
pub struct DiscoveredPattern {
    pub id: u64,
    pub centroid: Vec<f32>,
    pub experience_count: u64,
    pub average_reward: f32,
    pub positive_count: u64,
    pub negative_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiscoveryResult {
    ExistingPattern(u64),
    NewPattern(u64),
}

#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub similarity_threshold: f32,
    pub learning_rate: f32,
    pub reward_decay: f32,
    pub max_patterns: usize,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.90,
            learning_rate: 0.05,
            reward_decay: 0.95,
            max_patterns: 100_000,
        }
    }
}

#[derive(Debug)]
pub struct StrategicMemory {
    patterns: HashMap<u64, DiscoveredPattern>,
    config: DiscoveryConfig,
    next_id: u64,
    embedding_dimension: Option<usize>,
}

impl StrategicMemory {
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            patterns: HashMap::new(),
            config,
            next_id: 0,
            embedding_dimension: None,
        }
    }

    pub fn observe(
        &mut self,
        experience: StrategicExperience,
    ) -> Option<DiscoveryResult> {
        if experience.embedding.is_empty()
            || !experience.reward.is_finite()
            || experience.embedding.iter().any(|v| !v.is_finite())
        {
            return None;
        }

        match self.embedding_dimension {
            None => {
                self.embedding_dimension =
                    Some(experience.embedding.len());
            }
            Some(dimension) if dimension != experience.embedding.len() => {
                return None;
            }
            _ => {}
        }

        let best = self.find_best_pattern(&experience.embedding);

        match best {
            Some((id, similarity))
                if similarity >= self.config.similarity_threshold =>
            {
                self.update_pattern(id, &experience);

                Some(DiscoveryResult::ExistingPattern(id))
            }

            _ => self.create_pattern(experience),
        }
    }

    fn find_best_pattern(
        &self,
        embedding: &[f32],
    ) -> Option<(u64, f32)> {
        self.patterns
            .values()
            .filter_map(|pattern| {
                cosine_similarity(
                    embedding,
                    &pattern.centroid,
                )
                .map(|similarity| (pattern.id, similarity))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }

    fn create_pattern(
        &mut self,
        experience: StrategicExperience,
    ) -> Option<DiscoveryResult> {
        if self.patterns.len() >= self.config.max_patterns {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let pattern = DiscoveredPattern {
            id,
            centroid: experience.embedding,
            experience_count: 1,
            average_reward: experience.reward,
            positive_count: u64::from(experience.reward > 0.0),
            negative_count: u64::from(experience.reward < 0.0),
        };

        self.patterns.insert(id, pattern);

        Some(DiscoveryResult::NewPattern(id))
    }

    fn update_pattern(
        &mut self,
        id: u64,
        experience: &StrategicExperience,
    ) {
        if let Some(pattern) = self.patterns.get_mut(&id) {
            let lr = self.config.learning_rate;

            for (center, value) in pattern
                .centroid
                .iter_mut()
                .zip(experience.embedding.iter())
            {
                *center = (1.0 - lr) * *center + lr * *value;
            }

            pattern.experience_count += 1;

            let decay = self.config.reward_decay;

            pattern.average_reward =
                decay * pattern.average_reward
                + (1.0 - decay) * experience.reward;

            if experience.reward > 0.0 {
                pattern.positive_count += 1;
            } else if experience.reward < 0.0 {
                pattern.negative_count += 1;
            }
        }
    }

    pub fn get_pattern(
        &self,
        id: u64,
    ) -> Option<&DiscoveredPattern> {
        self.patterns.get(&id)
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn patterns(&self) -> Vec<&DiscoveredPattern> {
        let mut result: Vec<_> = self.patterns.values().collect();
        result.sort_by_key(|pattern| pattern.id);
        result
    }

    pub fn best_pattern(&self) -> Option<&DiscoveredPattern> {
        self.patterns.values().max_by(|a, b| {
            a.average_reward.total_cmp(&b.average_reward)
        })
    }

    pub fn clear(&mut self) {
        self.patterns.clear();
        self.next_id = 0;
        self.embedding_dimension = None;
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a <= f32::EPSILON || norm_b <= f32::EPSILON {
        return None;
    }

    Some(dot / (norm_a.sqrt() * norm_b.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn experience(embedding: Vec<f32>, reward: f32)
        -> StrategicExperience
    {
        StrategicExperience { embedding, reward }
    }

    fn memory() -> StrategicMemory {
        StrategicMemory::new(DiscoveryConfig {
            similarity_threshold: 0.90,
            learning_rate: 0.05,
            reward_decay: 0.95,
            max_patterns: 100,
        })
    }

    #[test]
    fn discovers_first_pattern() {
        let mut memory = memory();

        let result = memory.observe(
            experience(vec![1.0, 0.0, 0.0], 1.0)
        );

        assert_eq!(result, Some(DiscoveryResult::NewPattern(0)));
        assert_eq!(memory.pattern_count(), 1);
    }

    #[test]
    fn groups_similar_experiences() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));

        let result = memory.observe(
            experience(vec![0.99, 0.01], 0.5)
        );

        assert_eq!(
            result,
            Some(DiscoveryResult::ExistingPattern(0))
        );

        assert_eq!(memory.pattern_count(), 1);
    }

    #[test]
    fn discovers_different_pattern() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));

        let result = memory.observe(
            experience(vec![0.0, 1.0], -1.0)
        );

        assert_eq!(result, Some(DiscoveryResult::NewPattern(1)));
        assert_eq!(memory.pattern_count(), 2);
    }

    #[test]
    fn updates_experience_count() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));
        memory.observe(experience(vec![0.99, 0.01], 0.5));

        assert_eq!(
            memory.get_pattern(0).unwrap().experience_count,
            2
        );
    }

    #[test]
    fn tracks_positive_and_negative_outcomes() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));
        memory.observe(experience(vec![0.99, 0.01], -1.0));

        let pattern = memory.get_pattern(0).unwrap();

        assert_eq!(pattern.positive_count, 1);
        assert_eq!(pattern.negative_count, 1);
    }

    #[test]
    fn rejects_invalid_reward() {
        let mut memory = memory();

        let result = memory.observe(
            experience(vec![1.0, 0.0], f32::NAN)
        );

        assert_eq!(result, None);
        assert_eq!(memory.pattern_count(), 0);
    }

    #[test]
    fn rejects_wrong_embedding_dimension() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));

        let result = memory.observe(
            experience(vec![1.0, 0.0, 0.0], 1.0)
        );

        assert_eq!(result, None);
        assert_eq!(memory.pattern_count(), 1);
    }

    #[test]
    fn respects_pattern_limit() {
        let mut memory = StrategicMemory::new(
            DiscoveryConfig {
                max_patterns: 1,
                ..DiscoveryConfig::default()
            }
        );

        memory.observe(experience(vec![1.0, 0.0], 1.0));

        let result = memory.observe(
            experience(vec![0.0, 1.0], -1.0)
        );

        assert_eq!(result, None);
        assert_eq!(memory.pattern_count(), 1);
    }

    #[test]
    fn finds_best_pattern() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 0.5));
        memory.observe(experience(vec![0.0, 1.0], 1.0));

        assert_eq!(
            memory.best_pattern().unwrap().id,
            1
        );
    }

    #[test]
    fn clears_memory() {
        let mut memory = memory();

        memory.observe(experience(vec![1.0, 0.0], 1.0));
        memory.clear();

        assert_eq!(memory.pattern_count(), 0);
    }
}
