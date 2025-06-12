// Answer 0

#[derive(Debug)]
enum BernoulliError {
    InvalidProbability,
}

impl std::fmt::Display for BernoulliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BernoulliError::InvalidProbability => "p is outside [0, 1] in Bernoulli distribution",
        })
    }
}

#[test]
fn test_invalid_probability_error() {
    let err = BernoulliError::InvalidProbability;
    let mut output = String::new();
    let result = err.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "p is outside [0, 1] in Bernoulli distribution");
}

