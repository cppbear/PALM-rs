// Answer 0

#[test]
fn test_weight_valid_index_first() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType; // Simplified for testing
    }

    let weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleType; // Example value, not actually used
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution,
    };

    assert_eq!(dist.weight(0), Some(1));
}

#[test]
fn test_weight_valid_index_middle() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType; // Simplified for testing
    }

    let weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleType; // Example value, not actually used
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution,
    };

    assert_eq!(dist.weight(1), Some(2));
}

#[test]
fn test_weight_valid_index_last() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType; // Simplified for testing
    }

    let weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleType; // Example value, not actually used
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution,
    };

    assert_eq!(dist.weight(2), Some(3));
}

#[test]
fn test_weight_index_out_of_bounds() {
    struct SampleType;
    impl SampleUniform for SampleType {
        type Sampler = SampleType; // Simplified for testing
    }

    let weights = vec![1, 2, 3];
    let total_weight = 6;
    let weight_distribution = SampleType; // Example value, not actually used
    let dist = WeightedIndex {
        cumulative_weights: weights.clone(),
        total_weight,
        weight_distribution,
    };

    assert_eq!(dist.weight(3), None);
}

