// Answer 0

#[test]
fn test_from_ratio_valid_case() {
    let result = from_ratio(2, 2);
    assert_eq!(result, Ok(Bernoulli { p_int: ALWAYS_TRUE }));
}

#[test]
fn test_from_ratio_zero_numerator() {
    let result = from_ratio(0, 3);
    assert_eq!(result, Ok(Bernoulli { p_int: 0 }));
}

#[test]
fn test_from_ratio_numerator_greater() {
    let result = from_ratio(3, 2);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_denominator_zero() {
    let result = from_ratio(1, 0);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_edge_case() {
    let result = from_ratio(0, 0);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

