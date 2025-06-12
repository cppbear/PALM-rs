// Answer 0

#[test]
fn test_update_weights_insufficient_non_zero() {
    struct MockSampleUniform;
    impl SampleUniform for MockSampleUniform {
        type Sampler = MockSampler;
    }

    struct MockSampler;
    impl UniformSampler for MockSampler {
        type X = MockSampleUniform;
        fn new(_: MockSampleUniform, _: MockSampleUniform) -> Result<Self, ()> {
            Ok(MockSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![MockSampleUniform, MockSampleUniform, MockSampleUniform],
        total_weight: MockSampleUniform,
        weight_distribution: MockSampler,
    };

    // We will provide new_weights such that they do not increase total_weight
    let new_weights: Vec<(usize, &MockSampleUniform)> = vec![(0, &MockSampleUniform)];

    // Invoke the function under test and check the result
    let result = weighted_index.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_update_weights_error_invalid_weight() {
    struct MockSampleUniform;
    impl SampleUniform for MockSampleUniform {
        type Sampler = MockSampler;
    }

    struct MockSampler;
    impl UniformSampler for MockSampler {
        type X = MockSampleUniform;
        fn new(_: MockSampleUniform, _: MockSampleUniform) -> Result<Self, ()> {
            Ok(MockSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![MockSampleUniform, MockSampleUniform],
        total_weight: MockSampleUniform,
        weight_distribution: MockSampler,
    };

    // We provide a weight that is not valid (negative or NaN)
    let new_weights = vec![(0, &MockSampleUniform), (1, &MockSampleUniform)];

    // Invoke the function under test and check the result
    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_err());
} 

#[test]
fn test_update_weights_error_invalid_input() {
    struct MockSampleUniform;
    impl SampleUniform for MockSampleUniform {
        type Sampler = MockSampler;
    }

    struct MockSampler;
    impl UniformSampler for MockSampler {
        type X = MockSampleUniform;
        fn new(_: MockSampleUniform, _: MockSampleUniform) -> Result<Self, ()> {
            Ok(MockSampler)
        }
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![MockSampleUniform, MockSampleUniform],
        total_weight: MockSampleUniform,
        weight_distribution: MockSampler,
    };

    // We provide new_weights with indices that are not ordered or out of bounds
    let new_weights = vec![(1, &MockSampleUniform), (0, &MockSampleUniform)];

    // Invoke the function under test and check the result
    let result = weighted_index.update_weights(&new_weights);
    assert_eq!(result, Err(Error::InvalidInput));
}

