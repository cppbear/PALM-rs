// Answer 0

#[test]
fn test_new_weighted_index_empty_weights() {
    let empty_weights: Vec<f64> = vec![];
    let result = WeightedIndex::new(empty_weights);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_new_weighted_index_negative_weight() {
    let weights = vec![-1.0, 1.0];
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_weighted_index_nan_weight() {
    let weights = vec![std::f64::NAN, 1.0];
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_weighted_index_zero_sum_weights() {
    let weights = vec![0.0, 0.0];
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_new_weighted_index_overflow() {
    let weights = vec![std::u64::MAX, 1];
    let result = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::Overflow));
}

#[test]
fn test_new_weighted_index_valid_weights() {
    let weights = vec![1.0, 2.0, 3.0];
    let result = WeightedIndex::new(weights);
    assert!(result.is_ok());
}

