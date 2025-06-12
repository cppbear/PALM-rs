// Answer 0

#[test]
fn test_probability_zero() {
    let bernoulli = Bernoulli { p_int: 0 };
    let probability = bernoulli.p();
    assert_eq!(probability, 0.0);
}

#[test]
fn test_probability_half() {
    let bernoulli = Bernoulli { p_int: (SCALE / 2.0) as u64 };
    let probability = bernoulli.p();
    assert_eq!(probability, 0.5);
}

#[test]
fn test_probability_exact_one() {
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE - 1 };
    let probability = bernoulli.p();
    assert!((probability - 0.9999999999999999).abs() < f64::EPSILON);
}

#[test]
fn test_probability_almost_one() {
    let bernoulli = Bernoulli { p_int: (SCALE - 1.0) as u64 };
    let probability = bernoulli.p();
    assert!((probability - (SCALE - 1.0) / SCALE).abs() < f64::EPSILON);
}

