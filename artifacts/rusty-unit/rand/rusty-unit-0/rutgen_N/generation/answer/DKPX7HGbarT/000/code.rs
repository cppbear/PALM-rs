// Answer 0

#[test]
fn test_bernoulli_new_valid_probability() {
    struct Bernoulli {
        p_int: u64,
    }
    struct BernoulliError;

    const SCALE: f64 = (1u64 << 64) as f64; 
    const ALWAYS_TRUE: u64 = 1u64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: ALWAYS_TRUE });
            }
            return Err(BernoulliError);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    assert!(new(0.25).is_ok());
    assert!(new(0.75).is_ok());
    assert!(new(1.0).is_ok());
}

#[test]
fn test_bernoulli_new_invalid_probability() {
    struct Bernoulli {
        p_int: u64,
    }
    struct BernoulliError;

    const SCALE: f64 = (1u64 << 64) as f64;
    const ALWAYS_TRUE: u64 = 1u64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: ALWAYS_TRUE });
            }
            return Err(BernoulliError);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    assert!(new(-0.1).is_err());
    assert!(new(1.1).is_err());
}

#[test]
fn test_bernoulli_new_boundary_conditions() {
    struct Bernoulli {
        p_int: u64,
    }
    struct BernoulliError;

    const SCALE: f64 = (1u64 << 64) as f64;
    const ALWAYS_TRUE: u64 = 1u64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: ALWAYS_TRUE });
            }
            return Err(BernoulliError);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    assert!(new(0.0).is_ok());
    assert!(new(1.0).is_ok());
}

