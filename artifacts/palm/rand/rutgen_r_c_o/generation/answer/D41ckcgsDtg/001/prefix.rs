// Answer 0

#[test]
fn test_bernoulli_error_invalid_probability() {
    let error = BernoulliError::InvalidProbability;
    let mut buffer = String::new();
    let _ = error.fmt(&mut fmt::Formatter::new(&mut buffer));
}

#[test]
#[should_panic]
fn test_bernoulli_error_invalid_probability_with_no_format() {
    let error = BernoulliError::InvalidProbability;
    let _ = error.fmt(&mut fmt::Formatter::new(&mut String::new()));
}

