// Answer 0

#[test]
fn test_weight_valid_indices() {
    use crate::distr::uniform::UniformSampler as Sampler;
    use crate::distr::{Distribution, SampleUniform};

    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = usize;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![0, 1, 2];
    let total_weight = 3;
    let dist = WeightedIndex {
        cumulative_weights: cumulative_weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };
    
    assert_eq!(dist.weight(0), Some(0));
    assert_eq!(dist.weight(1), Some(1));
    assert_eq!(dist.weight(2), Some(2));
}

#[test]
fn test_weight_out_of_bounds() {
    use crate::distr::uniform::UniformSampler as Sampler;
    use crate::distr::{Distribution, SampleUniform};

    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = usize;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![0, 1, 2];
    let total_weight = 3;
    let dist = WeightedIndex {
        cumulative_weights: cumulative_weights.clone(),
        total_weight,
        weight_distribution: TestSampler,
    };
    
    assert_eq!(dist.weight(3), None);
}

