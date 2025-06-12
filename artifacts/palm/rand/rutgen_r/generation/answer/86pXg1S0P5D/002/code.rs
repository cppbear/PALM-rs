// Answer 0

#[test]
fn test_bernoulli_invalid_probability_denominator_zero() {
    let numerator = 1;
    let denominator = 0;
    let result = from_ratio(numerator, denominator);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_invalid_probability_numerator_greater_than_denominator() {
    let numerator = 2;
    let denominator = 1;
    let result = from_ratio(numerator, denominator);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_valid_probability_equal_numerator_denominator() {
    let numerator = 2;
    let denominator = 2;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().p_int, ALWAYS_TRUE);
}

#[test]
fn test_bernoulli_valid_probability_less_than_one() {
    let numerator = 1;
    let denominator = 3;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_ok());
}

