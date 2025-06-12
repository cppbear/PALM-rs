// Answer 0

#[test]
fn test_bernoulli_new_with_valid_probability() {
    struct Bernoulli {
        p_int: u64,
    }
    
    enum BernoulliError {
        InvalidProbability,
    }
    
    const ALWAYS_TRUE: u64 = u64::MAX; // Assuming ALWAYS_TRUE is defined like this
    const SCALE: f64 = u64::MAX as f64; // Assuming SCALE matches this for conversion purpose
    
    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
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

    let result = new(1.0);
    assert!(result.is_ok());
    let bernoulli = result.unwrap();
    assert_eq!(bernoulli.p_int, ALWAYS_TRUE);
}

#[test]
fn test_bernoulli_new_with_invalid_probability() {
    struct Bernoulli {
        p_int: u64,
    }
    
    enum BernoulliError {
        InvalidProbability,
    }
    
    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: u64::MAX }); // Assuming ALWAYS_TRUE is defined as u64::MAX
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli {
            p_int: (p * 1u64 << 64) as u64, // Dummy scale for example
        })
    }

    let result = new(-0.1); // Out of bounds input
    assert!(result.is_err());
    
    let result2 = new(1.1); // Out of bounds input
    assert!(result2.is_err());
}

