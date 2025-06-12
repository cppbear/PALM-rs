// Answer 0

#[test]
fn test_bernoulli_error_display_invalid_probability() {
    let error = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    
    assert_eq!(result, Ok(()));
    assert_eq!(output, "p is outside [0, 1] in Bernoulli distribution");
}

