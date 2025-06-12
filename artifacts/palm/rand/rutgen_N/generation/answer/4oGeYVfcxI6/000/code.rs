// Answer 0

#[test]
fn test_probability_always_true() {
    struct Bernoulli {
        p_int: u32,
    }
    
    const ALWAYS_TRUE: u32 = 1;
    const SCALE: f64 = 1_000_000.0;

    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.p(), 1.0);
}

#[test]
fn test_probability_zero() {
    struct Bernoulli {
        p_int: u32,
    }
    
    const SCALE: f64 = 1_000_000.0;

    let bernoulli = Bernoulli { p_int: 0 };
    assert_eq!(bernoulli.p(), 0.0);
}

#[test]
fn test_probability_half() {
    struct Bernoulli {
        p_int: u32,
    }
    
    const SCALE: f64 = 1_000_000.0;

    let bernoulli = Bernoulli { p_int: 500_000 };
    assert_eq!(bernoulli.p(), 0.5);
}

#[test]
fn test_probability_full() {
    struct Bernoulli {
        p_int: u32,
    }
    
    const SCALE: f64 = 1_000_000.0;

    let bernoulli = Bernoulli { p_int: 1_000_000 };
    assert_eq!(bernoulli.p(), 1.0);
}

