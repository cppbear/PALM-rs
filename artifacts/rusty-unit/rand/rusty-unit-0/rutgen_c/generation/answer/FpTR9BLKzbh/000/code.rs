// Answer 0

#[test]
fn test_weighted_index_total_weight() {
    struct DummySampler;
    
    impl UniformSampler for DummySampler {
        type X = f64; // Suitable type for weights
    }

    impl SampleUniform for f64 {
        type Sampler = DummySampler;
    }

    let cumulative_weights = vec![0.2, 0.3, 0.5];
    let total_weight = 1.0; 
    let weight_distribution = DummySampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    assert_eq!(weighted_index.total_weight(), 1.0);
}

#[test]
fn test_weighted_index_total_weight_boundary() {
    struct DummySampler;
    
    impl UniformSampler for DummySampler {
        type X = f64; // Suitable type for weights
    }

    impl SampleUniform for f64 {
        type Sampler = DummySampler;
    }

    let cumulative_weights = vec![];
    let total_weight = 0.0; 
    let weight_distribution = DummySampler;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    assert_eq!(weighted_index.total_weight(), 0.0);
}

