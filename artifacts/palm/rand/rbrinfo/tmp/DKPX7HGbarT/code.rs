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