// Answer 0

#[test]
fn test_weight_out_of_bounds_greater() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = usize;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = TestSampler; // Assuming this is correctly structured
    
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    // Testing for index greater than the cumulative weights length
    let result = weighted_index.weight(4);
    assert_eq!(result, None);
}

#[test]
fn test_weight_out_of_bounds_equal() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = usize;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = TestSampler; // Assuming this is correctly structured

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    // Testing for index equal to the cumulative weights length
    let result = weighted_index.weight(3);
    assert_eq!(result, Some(6));
}

#[test]
fn test_weight_out_of_bounds_less() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = usize;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = TestSampler; // Assuming this is correctly structured

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    // Testing for a valid index within bounds
    let result = weighted_index.weight(1);
    assert_eq!(result, Some(2));
}

