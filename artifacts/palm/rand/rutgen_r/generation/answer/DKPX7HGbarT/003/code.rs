// Answer 0

#[derive(Debug, PartialEq)]
struct Bernoulli {
    p_int: u64,
}

#[derive(Debug, PartialEq)]
enum BernoulliError {
    InvalidProbability,
}

const SCALE: f64 = 1u64 << 64; // Assuming this is the scale used in the original function
const ALWAYS_TRUE: u64 = u64::MAX; // Placeholder for the ALWAYS_TRUE constant

pub fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
    if !(0.0..1.0).contains(&p) {
        if p == 1.0 {
            return Ok(Bernoulli { p_int: ALWAYS_TRUE });
        }
        return Err(BernoulliError::InvalidProbability);
    }
    Ok(Bernoulli {
        p_int: (p * SCALE) as u64,
    })
}

#[test]
fn test_new_invalid_probability_negative() {
    let result = new(-0.1); 
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_new_invalid_probability_above_one() {
    let result = new(1.1); 
    assert_eq!(result, Err(BernoulliError::InvalidProbability));
}

#[test]
fn test_new_invalid_probability_exact_zero() {
    let result = new(0.0);
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_probability_exact_one() {
    let result = new(1.0);
    assert!(result.is_ok());
}

