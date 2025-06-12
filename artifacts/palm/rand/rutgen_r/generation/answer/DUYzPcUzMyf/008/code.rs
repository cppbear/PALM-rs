// Answer 0

#[test]
fn test_new_with_invalid_weight_empty() {
    let weights: Vec<f64> = vec![];
    let result: Result<WeightedIndex<f64>, Error> = new(weights);
    assert!(result.is_err());
    assert_eq!(result, Err(Error::InvalidInput));
}

#[test]
fn test_new_with_invalid_weight_negative() {
    let weights = vec![-1.0, 2.0];
    let result: Result<WeightedIndex<f64>, Error> = new(weights);
    assert!(result.is_err());
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_with_invalid_weight_nan() {
    let weights = vec![std::f64::NAN, 2.0];
    let result: Result<WeightedIndex<f64>, Error> = new(weights);
    assert!(result.is_err());
    assert_eq!(result, Err(Error::InvalidWeight));
}

