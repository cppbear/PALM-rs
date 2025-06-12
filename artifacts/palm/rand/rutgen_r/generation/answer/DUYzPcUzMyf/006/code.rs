// Answer 0

#[test]
fn test_weighted_index_invalid_input() {
    // Test for empty input
    let result: Result<WeightedIndex<f64>, Error> = WeightedIndex::new(vec![]);
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_weighted_index_negative_weight() {
    // Test when negative weights are present
    let weights = vec![1.0, -2.0];
    let result: Result<WeightedIndex<f64>, Error> = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_weighted_index_non_numerical_weight() {
    // Test with NaN weight
    let weights = vec![1.0, f64::NAN];
    let result: Result<WeightedIndex<f64>, Error> = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_weighted_index_zero_sum_weights() {
    // Test when the sum of weights is zero
    let weights = vec![0.0, 0.0, 0.0];
    let result: Result<WeightedIndex<f64>, Error> = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

#[test]
fn test_weighted_index_overflow() {
    // Test for weights that would cause an overflow
    let weights = vec![f64::MAX, f64::MAX];
    let result: Result<WeightedIndex<f64>, Error> = WeightedIndex::new(weights);
    assert_eq!(result, Err(Error::Overflow));
}

