// Answer 0

#[test]
fn test_bernoulli_error_display() {
    let error = BernoulliError::InvalidProbability;

    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "p is outside [0, 1] in Bernoulli distribution");
}

