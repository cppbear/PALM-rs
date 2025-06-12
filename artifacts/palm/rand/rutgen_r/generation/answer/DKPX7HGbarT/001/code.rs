// Answer 0

#[test]
fn test_bernoulli_new_valid_probability() {
    struct Bernoulli {
        p_int: u64,
    }

    #[derive(Debug)]
    enum BernoulliError {
        InvalidProbability,
    }

    const SCALE: f64 = 1u64 << 64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: u64::MAX });
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    assert_eq!(new(0.0).unwrap().p_int, 0);
    assert_eq!(new(0.5).unwrap().p_int, (0.5 * SCALE) as u64);
    assert_eq!(new(0.9999999999999999).unwrap().p_int, (0.9999999999999999 * SCALE) as u64);
    assert_eq!(new(1.0).unwrap().p_int, u64::MAX);
}

#[test]
#[should_panic]
fn test_bernoulli_new_invalid_probability_negative() {
    struct Bernoulli {
        p_int: u64,
    }

    #[derive(Debug)]
    enum BernoulliError {
        InvalidProbability,
    }

    const SCALE: f64 = 1u64 << 64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: u64::MAX });
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    // Testing with negative probability
    let _ = new(-0.1).expect("This should panic");
}

#[test]
#[should_panic]
fn test_bernoulli_new_invalid_probability_above_one() {
    struct Bernoulli {
        p_int: u64,
    }

    #[derive(Debug)]
    enum BernoulliError {
        InvalidProbability,
    }

    const SCALE: f64 = 1u64 << 64;

    fn new(p: f64) -> Result<Bernoulli, BernoulliError> {
        if !(0.0..1.0).contains(&p) {
            if p == 1.0 {
                return Ok(Bernoulli { p_int: u64::MAX });
            }
            return Err(BernoulliError::InvalidProbability);
        }
        Ok(Bernoulli {
            p_int: (p * SCALE) as u64,
        })
    }

    // Testing with a probability above one
    let _ = new(1.1).expect("This should panic");
}

