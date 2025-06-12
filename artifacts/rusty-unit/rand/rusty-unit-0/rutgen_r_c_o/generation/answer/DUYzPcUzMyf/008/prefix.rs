// Answer 0

#[test]
fn test_weighted_index_invalid_weight_negative() {
    let weights = vec![-1.0]; // Negative weight
    let result = WeightedIndex::<f64>::new(weights);
}

#[test]
fn test_weighted_index_invalid_weight_nan() {
    let weights = vec![f64::NAN]; // NaN weight
    let result = WeightedIndex::<f64>::new(weights);
}

