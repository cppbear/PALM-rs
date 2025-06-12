// Answer 0

#[test]
fn test_from_ratio_valid_case() {
    let result = Bernoulli::from_ratio(2, 3).unwrap();
    assert_eq!(result.p_int, ((2.0 / 3.0) * SCALE) as u64);
}

#[test]
fn test_from_ratio_full_success() {
    let result = Bernoulli::from_ratio(3, 3).unwrap();
    assert_eq!(result.p_int, ALWAYS_TRUE);
}

#[test]
fn test_from_ratio_zero_success() {
    let result = Bernoulli::from_ratio(0, 3).unwrap();
    assert_eq!(result.p_int, 0);
}

#[test]
fn test_from_ratio_invalid_numerator_greater_than_denominator() {
    let result = Bernoulli::from_ratio(4, 3);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_invalid_zero_denominator() {
    let result = Bernoulli::from_ratio(1, 0);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(BernoulliError::InvalidProbability));
}

