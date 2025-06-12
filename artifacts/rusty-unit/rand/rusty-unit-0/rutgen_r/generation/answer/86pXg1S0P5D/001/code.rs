// Answer 0

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    struct Bernoulli;
    struct BernoulliError;

    impl BernoulliError {
        const InvalidProbability: Self = BernoulliError;
    }

    fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError> {
        if numerator > denominator || denominator == 0 {
            return Err(BernoulliError::InvalidProbability);
        }
        if numerator == denominator {
            return Ok(Bernoulli);
        }
        let p_int = ((f64::from(numerator) / f64::from(denominator)) * 100.0) as u64; // Assume SCALE is 100.0
        Ok(Bernoulli)
    }

    // Test case where numerator > denominator
    let result = from_ratio(5, 3);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(BernoulliError::InvalidProbability));
}

