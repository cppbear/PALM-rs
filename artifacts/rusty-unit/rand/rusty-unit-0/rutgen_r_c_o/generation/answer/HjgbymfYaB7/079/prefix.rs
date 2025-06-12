// Answer 0

#[test]
fn test_update_weights_insufficient_non_zero() {
    use super::{WeightedIndex, Error, SampleUniform};

    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 1.0, 1.0],
        total_weight: 3.0,
        weight_distribution: TestSampler,
    };

    let new_weights: Vec<(usize, &f64)> = vec![(0, &1.0), (1, &1.0), (2, &0.0)];

    let result = weighted_index.update_weights(&new_weights);
}

#[test]
fn test_update_weights_invalid_weight() {
    use super::{WeightedIndex, Error, SampleUniform};

    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 1.0],
        total_weight: 2.0,
        weight_distribution: TestSampler,
    };

    let new_weights: Vec<(usize, &f64)> = vec![(0, &1.0), (1, &-1.0)];

    let result = weighted_index.update_weights(&new_weights);
}

#[test]
fn test_update_weights_invalid_input() {
    use super::{WeightedIndex, Error, SampleUniform};

    struct TestSampler;
    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 2.0],
        total_weight: 3.0,
        weight_distribution: TestSampler,
    };

    let new_weights: Vec<(usize, &f64)> = vec![(1, &1.0), (0, &2.0)];

    let result = weighted_index.update_weights(&new_weights);
}

