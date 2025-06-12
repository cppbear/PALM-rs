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
fn test_bernoulli_error_invalid_probability_display() {
    let error = BernoulliError::InvalidProbability;
    let result = format!("{}", error);
    assert_eq!(result, "p is outside [0, 1] in Bernoulli distribution");
}

