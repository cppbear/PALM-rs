// Answer 0

#[test]
fn test_bernoulli_new_invalid_probability_negative() {
    let result = Bernoulli::new(-0.1);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_new_invalid_probability_above_one() {
    let result = Bernoulli::new(1.1);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_new_zero() {
    let result = Bernoulli::new(0.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().p_int, 0);
}

#[test]
fn test_bernoulli_new_one() {
    let result = Bernoulli::new(1.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().p_int, ALWAYS_TRUE);
}

