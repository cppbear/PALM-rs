// Answer 0

#[test]
fn test_weight_with_valid_index() {
    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleUniformImpl;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let _ = weighted_index.weight(1);
}

#[test]
fn test_weight_with_index_zero() {
    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleUniformImpl;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };
    
    let _ = weighted_index.weight(0);
}

#[test]
fn test_weight_with_index_in_bounds() {
    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![2, 4, 6];
    let total_weight = 12;
    let weight_distribution = SampleUniformImpl;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let _ = weighted_index.weight(1);
}

#[test]
fn test_weight_with_multiple_valid_inputs() {
    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![5, 10, 15, 20];
    let total_weight = 50;
    let weight_distribution = SampleUniformImpl;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let _ = weighted_index.weight(2);
}

#[test]
fn test_weight_with_index_on_edge() {
    struct SampleUniformImpl;
    impl SampleUniform for SampleUniformImpl {
        type Sampler = ();
    }
    
    let cumulative_weights = vec![1, 1, 1, 1];
    let total_weight = 4;
    let weight_distribution = SampleUniformImpl;

    let weighted_index = WeightedIndex {
        cumulative_weights,
        total_weight,
        weight_distribution,
    };

    let _ = weighted_index.weight(3);
}

