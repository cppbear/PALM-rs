// Answer 0

#[test]
fn test_weight_index_with_valid_bounds() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = usize;
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
            0 // Dummy implementation for the sampler
        }
    }

    struct TestSampleUniform;
    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![0, 1, 2];
    let total_weight = 3;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    // Test case where index is within bounds and equals to 0
    assert_eq!(weighted_index.weight(0), Some(0));

    // Test case where index is exactly equal to the length of cumulative weights
    assert_eq!(weighted_index.weight(3), Some(3));
}

#[test]
fn test_weight_index_with_out_of_bounds() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = usize;
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
            0 // Dummy implementation for the sampler
        }
    }

    struct TestSampleUniform;
    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![0, 1, 2];
    let total_weight = 3;
    let weight_distribution = TestSampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    // Test case where index is greater than the cumulative weights
    assert_eq!(weighted_index.weight(4), None);
}

