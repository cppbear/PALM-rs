// Answer 0

#[derive(Debug)]
struct Bernoulli {
    p_int: u64,
}

#[derive(Debug)]
enum BernoulliError {
    InvalidProbability,
}

const SCALE: f64 = 1_000_000.0;
const ALWAYS_TRUE: u64 = 1;

pub fn from_ratio(numerator: u32, denominator: u32) -> Result<Bernoulli, BernoulliError> {
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
fn test_from_ratio_valid() {
    let result = from_ratio(2, 3);
    assert!(result.is_ok());
    if let Ok(bernoulli) = result {
        assert_eq!(bernoulli.p_int, (2.0 / 3.0 * SCALE) as u64);
    }
}

#[test]
fn test_from_ratio_always_true() {
    let result = from_ratio(3, 3);
    assert!(result.is_ok());
    if let Ok(bernoulli) = result {
        assert_eq!(bernoulli.p_int, ALWAYS_TRUE);
    }
}

#[test]
fn test_from_ratio_always_false() {
    let result = from_ratio(0, 3);
    assert!(result.is_ok());
    if let Ok(bernoulli) = result {
        assert_eq!(bernoulli.p_int, 0);
    }
}

#[test]
fn test_from_ratio_invalid_probability_numerator_greater_than_denominator() {
    let result = from_ratio(4, 3);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

#[test]
fn test_from_ratio_invalid_probability_denominator_zero() {
    let result = from_ratio(1, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

