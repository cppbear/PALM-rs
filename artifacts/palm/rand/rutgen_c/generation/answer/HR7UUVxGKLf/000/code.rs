// Answer 0

#[test]
fn test_weights_initial() {
    struct MockSampler;
    impl UniformSampler for MockSampler {
        type X = i32;
    }
    
    struct MockSampleUniform;
    impl SampleUniform for MockSampleUniform {
        type Sampler = MockSampler;
    }

    let weights = vec![1, 2, 3];
    let cumulative_weights = weights.clone();
    let total_weight = weights.iter().sum();
    
    let dist = WeightedIndex::<MockSampleUniform> {
        cumulative_weights,
        total_weight,
        weight_distribution: MockSampler,
    };

    let weights_iter: Vec<_> = dist.weights().collect();
    assert_eq!(weights_iter, vec![1, 2, 3]);
}

#[test]
fn test_weights_after_update() {
    struct MockSampler;
    impl UniformSampler for MockSampler {
        type X = i32;
    }
    
    struct MockSampleUniform;
    impl SampleUniform for MockSampleUniform {
        type Sampler = MockSampler;
    }

    let weights = vec![1, 2, 3];
    let cumulative_weights = weights.clone();
    let total_weight = weights.iter().sum();

    let mut dist = WeightedIndex::<MockSampleUniform> {
        cumulative_weights,
        total_weight,
        weight_distribution: MockSampler,
    };

    // Simulate an update of weights as if the `update_weights` method is called
    let updated_weights = vec![2, 2, 3];
    dist.cumulative_weights = updated_weights.clone();
    dist.total_weight = updated_weights.iter().sum();

    let weights_iter: Vec<_> = dist.weights().collect();
    assert_eq!(weights_iter, vec![2, 2, 3]);
}

