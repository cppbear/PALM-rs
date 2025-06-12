// Answer 0

#[test]
fn test_update_weights_invalid_weight() {
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![1.0, 2.0, 3.0],
        total_weight: 6.0,
        weight_distribution: <f64 as SampleUniform>::Sampler::new(0.0, 6.0).unwrap(),
    };

    let new_weights = vec![(1, &-1.0)];
    let _result = weighted_index.update_weights(&new_weights);
}

#[test]
fn test_update_weights_invalid_weight_multiple() {
    let mut weighted_index = WeightedIndex {
        cumulative_weights: vec![2.0, 4.0, 6.0, 8.0],
        total_weight: 20.0,
        weight_distribution: <f64 as SampleUniform>::Sampler::new(0.0, 20.0).unwrap(),
    };

    let new_weights = vec![(2, &-3.0), (3, &-2.0)];
    let _result = weighted_index.update_weights(&new_weights);
}

