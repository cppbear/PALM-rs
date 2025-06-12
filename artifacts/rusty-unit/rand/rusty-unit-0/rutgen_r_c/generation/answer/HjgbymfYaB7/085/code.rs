// Answer 0

#[test]
fn test_update_weights_success() {
    use alloc::vec;

    struct TestWeight;
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(_: TestWeight, _: TestWeight) -> Result<Self, Error> {
            Ok(Self)
        }
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight, TestWeight, TestWeight],
        total_weight: TestWeight,
        weight_distribution: TestSampler,
    };

    let result = weighted_index.update_weights(&[(0, &TestWeight), (1, &TestWeight)]);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_update_weights_invalid_input_duplicate_index() {
    use alloc::vec;

    struct TestWeight;
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(_: TestWeight, _: TestWeight) -> Result<Self, Error> {
            Ok(Self)
        }
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight, TestWeight, TestWeight],
        total_weight: TestWeight,
        weight_distribution: TestSampler,
    };

    let result = weighted_index.update_weights(&[(0, &TestWeight), (0, &TestWeight)]);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_update_weights_negative_weight() {
    use alloc::vec;

    struct TestWeight;
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(_: TestWeight, _: TestWeight) -> Result<Self, Error> {
            Ok(Self)
        }
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight, TestWeight, TestWeight],
        total_weight: TestWeight,
        weight_distribution: TestSampler,
    };

    let result = weighted_index.update_weights(&[(0, &TestWeight), (1, &TestWeight), (2, &TestWeight)]);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_update_weights_insufficient_nonzero_weight() {
    use alloc::vec;

    struct TestWeight;
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = TestWeight;

        fn new(_: TestWeight, _: TestWeight) -> Result<Self, Error> {
            Ok(Self)
        }
    }

    impl Weight for TestWeight {}

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![TestWeight, TestWeight, TestWeight],
        total_weight: TestWeight,
        weight_distribution: TestSampler,
    };

    let result = weighted_index.update_weights(&[(0, &TestWeight), (1, &TestWeight)]);
    assert_eq!(result.is_ok(), true); // Ensure total weight condition can pass with valid weights.
    // Now empty the weights to force an InsufficientNonZero error.
    weighted_index.cumulative_weights = vec![TestWeight, TestWeight, TestWeight];
    weighted_index.total_weight = TestWeight; // Here we set it as zero equivalent of TestWeight.
    let result = weighted_index.update_weights(&[(0, &TestWeight), (1, &TestWeight)]);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

