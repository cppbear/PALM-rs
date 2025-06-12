// Answer 0

#[test]
fn test_from_ratio_invalid_probability_greater_than_denominator() {
    let numerator = 2;
    let denominator = 1;
    let result = from_ratio(numerator, denominator);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_invalid_probability_denominator_zero() {
    let numerator = 1;
    let denominator = 0;
    let result = from_ratio(numerator, denominator);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_from_ratio_valid_equal_numerator_denominator() {
    let numerator = 2;
    let denominator = 2;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_ok());
    if let Ok(b) = result {
        assert_eq!(b.p_int, ALWAYS_TRUE);
    }
}

#[test]
fn test_from_ratio_valid_numerator_less_than_denominator() {
    let numerator = 1;
    let denominator = 2;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_ok());
    if let Ok(b) = result {
        assert!(b.p_int < ALWAYS_TRUE);
    }
}

