// Answer 0

#[test]
fn test_update_weights_success() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = f64;
        
        fn new(low: Self::X, high: Self::X) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }
    
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 3.0], // Cumulative weights for 3 weights
        total_weight: 3.0,
        weight_distribution: TestSampler::new(0.0, 3.0).unwrap(),
    };
    
    let new_weights = vec![(0, &2.0), (2, &1.0)];
    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_ok());
    assert_eq!(weighted_index.cumulative_weights, vec![0.0, 2.0, 3.0]);
    assert_eq!(weighted_index.total_weight, 3.0);
}

#[test]
fn test_update_weights_invalid_input() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = f64;

        fn new(low: Self::X, high: Self::X) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }
    
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 3.0],
        total_weight: 3.0,
        weight_distribution: TestSampler::new(0.0, 3.0).unwrap(),
    };
    
    let new_weights = vec![(1, &2.0), (0, &3.0)]; // Incorrect order
    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidInput));
}

#[test]
fn test_update_weights_negative_weight() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = f64;

        fn new(low: Self::X, high: Self::X) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }
    
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 1.0, 3.0],
        total_weight: 3.0,
        weight_distribution: TestSampler::new(0.0, 3.0).unwrap(),
    };
    
    let new_weights = vec![(0, &-1.0)]; // Negative weight
    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InvalidWeight));
}

#[test]
fn test_update_weights_insufficient_non_zero() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = f64;

        fn new(low: Self::X, high: Self::X) -> Result<Self, ()> {
            Ok(TestSampler)
        }
    }
    
    struct TestWeight(f64);
    impl SampleUniform for TestWeight {
        type Sampler = TestSampler;
    }

    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![0.0, 0.0, 0.0], // All zero weights
        total_weight: 0.0,
        weight_distribution: TestSampler::new(0.0, 0.0).unwrap(),
    };
    
    let new_weights = vec![(0, &1.0)];
    let result = weighted_index.update_weights(&new_weights);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::InsufficientNonZero));
}

