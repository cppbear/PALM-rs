// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::thread_rng;
    use rand::distributions::{Distribution, WeightedIndex};

    struct WeightedSampler {
        weight_distribution: WeightedIndex<u32>,
        cumulative_weights: Vec<u32>,
    }

    impl WeightedSampler {
        fn new(weights: &[u32]) -> Self {
            let weight_distribution = WeightedIndex::new(weights).unwrap();
            let cumulative_weights = weights.to_vec();
            Self {
                weight_distribution,
                cumulative_weights,
            }
        }

        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
            let chosen_weight = self.weight_distribution.sample(rng);
            self.cumulative_weights
                .partition_point(|w| w <= &chosen_weight)
        }
    }

    #[test]
    fn test_sample_with_equal_weights() {
        let weights = [1, 1, 1];
        let sampler = WeightedSampler::new(&weights);
        let mut rng = thread_rng();

        let result = sampler.sample(&mut rng);
        assert!(result < weights.len());
    }

    #[test]
    fn test_sample_with_increasing_weights() {
        let weights = [1, 2, 3, 4, 5];
        let sampler = WeightedSampler::new(&weights);
        let mut rng = thread_rng();

        for _ in 0..100 {
            let result = sampler.sample(&mut rng);
            assert!(result < weights.len());
        }
    }

    #[test]
    fn test_sample_with_decreasing_weights() {
        let weights = [5, 4, 3, 2, 1];
        let sampler = WeightedSampler::new(&weights);
        let mut rng = thread_rng();

        for _ in 0..100 {
            let result = sampler.sample(&mut rng);
            assert!(result < weights.len());
        }
    }

    #[test]
    fn test_sample_with_all_zero_weights() {
        let weights = [0, 0, 0];
        let sampler = WeightedSampler::new(&weights);
        let mut rng = thread_rng();

        let result = sampler.sample(&mut rng);
        assert_eq!(result, 0);
    }

    #[should_panic]
    #[test]
    fn test_sample_with_empty_weights() {
        let weights: Vec<u32> = vec![];
        let sampler = WeightedSampler::new(&weights); // This should panic
        let mut rng = thread_rng();

        let _result = sampler.sample(&mut rng);
    }
}

