// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::ThreadRng;
    use rand::Rng;

    struct WeightedSample {
        weight_distribution: Vec<usize>,
        cumulative_weights: Vec<usize>,
    }

    impl WeightedSample {
        fn new(weights: Vec<usize>) -> Self {
            let cumulative_weights = weights.iter().scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            }).collect();
            WeightedSample {
                weight_distribution: weights,
                cumulative_weights,
            }
        }

        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
            let chosen_weight = self.weight_distribution.choose(rng).unwrap();
            self.cumulative_weights
                .partition_point(|w| w <= &chosen_weight)
        }
    }

    #[test]
    fn test_sample_with_single_weight() {
        let weights = vec![10];
        let mut rng = rand::thread_rng();
        let sample = WeightedSample::new(weights);
        assert_eq!(sample.sample(&mut rng), 0);
    }

    #[test]
    fn test_sample_with_multiple_weights() {
        let weights = vec![10, 20, 30];
        let mut rng = rand::thread_rng();
        let sample = WeightedSample::new(weights);
        let samples: Vec<_> = (0..100).map(|_| sample.sample(&mut rng)).collect();
        assert!(samples.iter().any(|&s| s == 0));
        assert!(samples.iter().any(|&s| s == 1));
        assert!(samples.iter().any(|&s| s == 2));
    }

    #[test]
    fn test_sample_with_edge_case() {
        let weights = vec![1, 0, 1];
        let mut rng = rand::thread_rng();
        let sample = WeightedSample::new(weights);
        let samples: Vec<_> = (0..100).map(|_| sample.sample(&mut rng)).collect();
        assert!(samples.iter().any(|&s| s == 0 || s == 2)); // 1 should not be sampled since it has weight 0
    }

    #[should_panic]
    fn test_sample_with_empty_weights() {
        let weights: Vec<usize> = vec![];
        let mut rng = rand::thread_rng();
        let sample = WeightedSample::new(weights);
        sample.sample(&mut rng); // This should panic
    }
}

