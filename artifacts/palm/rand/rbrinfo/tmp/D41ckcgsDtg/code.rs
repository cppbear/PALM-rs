fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BernoulliError::InvalidProbability => "p is outside [0, 1] in Bernoulli distribution",
        })
    }