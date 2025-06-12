// Answer 0

#[test]
fn test_new_invalid_input_empty_iterator() {
    let result: Result<WeightedIndex<f64>, Error> = new(vec![]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidInput);
}

#[test]
fn test_new_invalid_weight_nan() {
    let result: Result<WeightedIndex<f64>, Error> = new(vec![f64::NAN]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidWeight);
}

#[test]
fn test_new_invalid_weight_negative() {
    let result: Result<WeightedIndex<f64>, Error> = new(vec![-1.0]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidWeight);
}

#[test]
fn test_new_insufficient_non_zero() {
    let result: Result<WeightedIndex<f64>, Error> = new(vec![0.0, 0.0]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InsufficientNonZero);
}

#[test]
fn test_new_overflow() {
    let result: Result<WeightedIndex<u32>, Error> = new(vec![u32::MAX, 1]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::Overflow);
}

