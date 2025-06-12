// Answer 0

#[test]
fn test_weighted_index_iter_debug_valid() {
    use crate::distr::uniform::Uniform;
    use crate::Rng;

    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights = vec![1.0, 2.0, 3.0]; // total_weight = 6.0
    let total_weight = 6.0;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = 1; // valid index
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };

    let _ = fmt(&iter, &mut fmt::Formatter::new()); // Call fmt to execute the function
}

#[test]
fn test_weighted_index_iter_debug_edge_case() {
    use crate::distr::uniform::Uniform;
    use crate::Rng;

    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights = vec![0.0]; // total_weight = 0.0
    let total_weight = 0.0;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = 0; // valid index
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };

    let _ = fmt(&iter, &mut fmt::Formatter::new()); // Call fmt to execute the function
}

#[test]
#[should_panic]
fn test_weighted_index_iter_debug_invalid_index() {
    use crate::distr::uniform::Uniform;
    use crate::Rng;

    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights = vec![1.0, 2.0]; // length is 2
    let total_weight = 3.0;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = 2; // invalid index, should panic
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };

    let _ = fmt(&iter, &mut fmt::Formatter::new()); // Call fmt to trigger panic
}

#[test]
fn test_weighted_index_iter_debug_empty_cumulative_weights() {
    use crate::distr::uniform::Uniform;
    use crate::Rng;

    struct TestSampler;

    impl SampleUniform for TestSampler {
        type Sampler = Self;
    }

    let cumulative_weights: Vec<f32> = vec![]; // no weights
    let total_weight = 0.0;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = 0; // valid index (no elements, should handle gracefully)
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };

    let _ = fmt(&iter, &mut fmt::Formatter::new()); // Call fmt to execute the function
}

