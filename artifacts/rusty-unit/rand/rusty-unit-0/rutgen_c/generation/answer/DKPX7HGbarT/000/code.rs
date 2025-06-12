// Answer 0

#[test]
fn test_bernoulli_new_valid_probability() {
    let prob = 0.3;
    let bernoulli = Bernoulli::new(prob).unwrap();
    assert_eq!(bernoulli.p_int, (prob * SCALE) as u64);
}

#[test]
fn test_bernoulli_new_probability_zero() {
    let prob = 0.0;
    let bernoulli = Bernoulli::new(prob).unwrap();
    assert_eq!(bernoulli.p_int, 0);
}

#[test]
fn test_bernoulli_new_probability_one() {
    let prob = 1.0;
    let bernoulli = Bernoulli::new(prob).unwrap();
    assert_eq!(bernoulli.p_int, ALWAYS_TRUE);
}

#[test]
fn test_bernoulli_new_invalid_probability_less_than_zero() {
    let prob = -0.1;
    let result = Bernoulli::new(prob);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

#[test]
fn test_bernoulli_new_invalid_probability_greater_than_one() {
    let prob = 1.1;
    let result = Bernoulli::new(prob);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

