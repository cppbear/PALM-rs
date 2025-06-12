// Answer 0

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let result = from_ratio(5, 3);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_numerator_equals_denominator() {
    let result = from_ratio(3, 3);
    assert_eq!(result, Ok(Bernoulli { p_int: ALWAYS_TRUE }));
}

#[test]
fn test_from_ratio_numerator_zero() {
    let result = from_ratio(0, 3);
    assert_eq!(result, Ok(Bernoulli { p_int: 0 }));
}

#[test]
fn test_from_ratio_denominator_zero() {
    let result = from_ratio(1, 0);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator_edge_case() {
    let result = from_ratio(10, 5);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

