// Answer 0

#[test]
fn test_bernoulli_new_valid_probability_true() {
    let probability = 1.0;
    let result = Bernoulli::new(probability);
    assert_eq!(result, Ok(Bernoulli { p_int: ALWAYS_TRUE }));
}

#[test]
fn test_bernoulli_new_invalid_probability_negative() {
    let probability = -0.1;
    let result = Bernoulli::new(probability);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_new_invalid_probability_above_one() {
    let probability = 1.1;
    let result = Bernoulli::new(probability);
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_bernoulli_new_zero_probability() {
    let probability = 0.0;
    let result = Bernoulli::new(probability);
    assert!(result.is_ok());
    let bernoulli = result.unwrap();
    assert_eq!(bernoulli.p_int, 0);
}

