// Answer 0

#[test]
fn test_sample_with_weights() {
    struct MockRng {
        value: usize,
    }

    impl Rng for MockRng {
        // Implement required methods here
    }

    struct MockSampler {
        weights: Vec<usize>,
    }

    impl UniformSampler for MockSampler {
        type X = usize;

        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
            // Here we can return a mock weight for testing purposes.
            self.weights[0] // Simplifying to just return the first weight for the sake of this test.
        }
    }

    impl SampleUniform for usize {
        type Sampler = MockSampler;
    }

    let weights = vec![10, 20, 30, 40];
    let cumulative_weights = vec![10, 30, 60, 100];
    let total_weight: usize = cumulative_weights.last().copied().unwrap_or(0);
    let weight_distribution = MockSampler { weights };

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let mut rng = MockRng { value: 0 };
    let result = weighted_index.sample(&mut rng);
    
    assert_eq!(result, 0); // expected index based on mock logic
}

#[test]
fn test_sample_with_different_weights() {
    struct MockRng {
        value: usize,
    }

    impl Rng for MockRng {
        // Implement required methods here
    }

    struct MockSampler {
        weights: Vec<usize>,
    }

    impl UniformSampler for MockSampler {
        type X = usize;

        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
            self.weights[1] // Return the second weight for this test
        }
    }

    impl SampleUniform for usize {
        type Sampler = MockSampler;
    }

    let weights = vec![10, 20, 30, 40];
    let cumulative_weights = vec![10, 30, 60, 100];
    let total_weight: usize = cumulative_weights.last().copied().unwrap_or(0);
    let weight_distribution = MockSampler { weights };

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let mut rng = MockRng { value: 1 };
    let result = weighted_index.sample(&mut rng);

    assert_eq!(result, 1); // expected index based on mock logic
}

