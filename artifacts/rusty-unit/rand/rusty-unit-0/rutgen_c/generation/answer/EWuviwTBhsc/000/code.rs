// Answer 0

#[test]
fn test_weighted_index_iter_debug_fmt() {
    struct TestSampler;
    
    impl UniformSampler for TestSampler {
        type X = f32;
    }

    struct TestSampleUniform;

    impl SampleUniform for TestSampleUniform {
        type Sampler = TestSampler;
    }

    let cumulative_weights = vec![1.0, 2.0, 3.0];
    let total_weight = 6.0;
    
    let weight_distribution = TestSampler;
    
    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let index = 1;
    let iter = WeightedIndexIter {
        weighted_index: &weighted_index,
        index,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", iter);
    
    assert!(result.is_ok());
    assert!(output.contains("WeightedIndexIter"));
    assert!(output.contains(&format!("{:?}", iter.weighted_index)));
    assert!(output.contains(&format!("{}", iter.index)));
}

