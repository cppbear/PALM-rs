// Answer 0

#[test]
fn test_update_weights_invalid_input() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = f32;
    }
    
    impl SampleUniform for f32 {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 2.0, 3.0],
        total_weight: 6.0,
        weight_distribution: TestSampler,
    };
    
    let new_weights: Vec<(usize, &f32)> = vec![(2, &0.0), (1, &1.0)];
    
    assert_eq!(weighted_index.update_weights(&new_weights), Err(Error::InvalidInput));
}

#[test]
fn test_update_weights_weight_negative() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = f32;
    }
    
    impl SampleUniform for f32 {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 2.0, 3.0],
        total_weight: 6.0,
        weight_distribution: TestSampler,
    };
    
    let new_weights: Vec<(usize, &f32)> = vec![(1, &-1.0)];
    
    assert_eq!(weighted_index.update_weights(&new_weights), Err(Error::InvalidWeight));
}

#[test]
fn test_update_weights_insufficient_non_zero() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = f32;
    }
    
    impl SampleUniform for f32 {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 1.0, 1.0],
        total_weight: 3.0,
        weight_distribution: TestSampler,
    };
    
    let new_weights: Vec<(usize, &f32)> = vec![(0, &0.0), (1, &0.0), (2, &0.0)];
    
    assert_eq!(weighted_index.update_weights(&new_weights), Err(Error::InsufficientNonZero));
}

#[test]
fn test_update_weights_index_too_large() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = f32;
    }
    
    impl SampleUniform for f32 {
        type Sampler = TestSampler;
    }
    
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 2.0],
        total_weight: 3.0,
        weight_distribution: TestSampler,
    };
    
    let new_weights: Vec<(usize, &f32)> = vec![(3, &1.0)];
    
    assert_eq!(weighted_index.update_weights(&new_weights), Err(Error::InvalidInput));
}

