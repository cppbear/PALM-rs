// Answer 0

#[test]
fn test_weighted_index_iter_debug_display() {
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = i32; // Using i32 as a sample type
    }

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let sampler = TestSampler;
    
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: sampler,
    };

    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 0,
    };

    let mut output = String::new();
    let result = fmt::write(&mut output, |f| iter.fmt(f));
    assert!(result.is_ok());
    assert!(output.contains("WeightedIndexIter"));
    assert!(output.contains("weighted_index"));
    assert!(output.contains("index"));
} 

#[test]
fn test_weighted_index_iter_debug_display_with_non_zero_index() {
    struct TestSampler;

    impl UniformSampler for TestSampler {
        type X = i32; // Using i32 as a sample type
    }

    impl SampleUniform for TestSampler {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![2, 4, 6];
    let total_weight = 12;
    let sampler = TestSampler;
    
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution: sampler,
    };

    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index: 1,
    };

    let mut output = String::new();
    let result = fmt::write(&mut output, |f| iter.fmt(f));
    assert!(result.is_ok());
    assert!(output.contains("WeightedIndexIter"));
    assert!(output.contains("weighted_index"));
    assert!(output.contains("index"));
}

