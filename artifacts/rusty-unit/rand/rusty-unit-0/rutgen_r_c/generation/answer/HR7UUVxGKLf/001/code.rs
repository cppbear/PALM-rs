// Answer 0

#[test]
fn test_weights_initial() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = i32;
    }

    impl SampleUniform for i32 {
        type Sampler = TestSampler;
    }

    let weights = vec![1, 2, 3];
    let total_weight = weights.iter().sum();
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };

    let mut iter = dist.weights();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_weights_after_update() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = i32;
    }

    impl SampleUniform for i32 {
        type Sampler = TestSampler;
    }

    let mut weights = vec![1, 2, 3];
    let total_weight = weights.iter().sum();
    let mut dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };

    weights[0] = 2; // update the first weight
    let total_weight_updated = weights.iter().sum();
    dist.cumulative_weights = weights.clone();
    dist.total_weight = total_weight_updated;

    let mut iter = dist.weights();
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![2, 2, 3]);
}

#[should_panic]
#[test]
fn test_weights_out_of_bounds() {
    struct TestSampler;
    impl UniformSampler for TestSampler {
        type X = i32;
    }

    impl SampleUniform for i32 {
        type Sampler = TestSampler;
    }

    let weights = vec![1, 2, 3];
    let total_weight = weights.iter().sum();
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };

    let mut iter = dist.weights();
    
    for _ in 0..5 {
        iter.next(); // assuming more calls than available weights could trigger a panic
    }
}

