// Answer 0

#[test]
fn test_from_ratio_valid_full_success() {
    let result = Bernoulli::from_ratio(2, 2);
    assert_eq!(result, Ok(Bernoulli { p_int: ALWAYS_TRUE }));
}

#[test]
fn test_from_ratio_invalid_denominator_zero() {
    let result = Bernoulli::from_ratio(1, 0);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_invalid_numerator_greater_denominator() {
    let result = Bernoulli::from_ratio(2, 1);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

