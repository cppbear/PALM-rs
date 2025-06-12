// Answer 0

#[derive(Debug)]
struct Bernoulli {
    p_int: u64,
}

#[derive(Debug)]
enum BernoulliError {
    InvalidProbability,
}

const SCALE: f64 = 100.0;
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
fn test_from_ratio_numerator_equals_denominator() {
    let numerator = 5;
    let denominator = 5;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_ok());
    let bernoulli = result.unwrap();
    assert_eq!(bernoulli.p_int, ALWAYS_TRUE);
}

#[test]
fn test_from_ratio_denominator_zero() {
    let numerator = 1;
    let denominator = 0;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

#[test]
fn test_from_ratio_numerator_greater_than_denominator() {
    let numerator = 3;
    let denominator = 2;
    let result = from_ratio(numerator, denominator);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), BernoulliError::InvalidProbability);
}

