// Answer 0

#[derive(Debug)]
struct Bernoulli {
    p_int: u64,
}

#[derive(Debug)]
enum BernoulliError {
    InvalidProbability,
}

const ALWAYS_TRUE: u64 = 1;
const SCALE: f64 = 1_000_000.0;

fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError> {
    if numerator > denominator || denominator == 0 {
        return Err(BernoulliError::InvalidProbability);
    }
    if numerator == denominator {
        return Ok(Bernoulli { p_int: ALWAYS_TRUE });
    }
    let p_int = ((f64::from(numerator) / f64::from(denominator)) * SCALE) as u64;
    Ok(Bernoulli { p_int })
}

#[test]
fn test_from_ratio_numerator_equals_denominator() {
    let result = from_ratio(3, 3);
    assert!(result.is_ok());
    if let Ok(bernoulli) = result {
        assert_eq!(bernoulli.p_int, ALWAYS_TRUE);
    }
}

#[test]
fn test_from_ratio_denominator_zero() {
    let result = from_ratio(2, 0);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, BernoulliError::InvalidProbability);
    }
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let result = from_ratio(4, 3);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, BernoulliError::InvalidProbability);
    }
}

#[test]
fn test_from_ratio_numerator_less_than_denominator() {
    let result = from_ratio(1, 3);
    assert!(result.is_ok());
    if let Ok(bernoulli) = result {
        assert_ne!(bernoulli.p_int, ALWAYS_TRUE);
    }
}

