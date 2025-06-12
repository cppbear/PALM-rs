// Answer 0

#[test]
fn test_probability_when_always_true() {
    let bernoulli = Bernoulli { p_int: ALWAYS_TRUE };
    assert_eq!(bernoulli.p(), 1.0);
}

#[test]
fn test_probability_when_not_always_true() {
    let bernoulli = Bernoulli { p_int: 1 };
    assert_eq!(bernoulli.p(), 1.0 / SCALE);
}

#[test]
fn test_probability_with_large_value() {
    let bernoulli = Bernoulli { p_int: u64::MAX - 1 };
    let expected_probability = (u64::MAX - 1) as f64 / SCALE;
    assert!((bernoulli.p() - expected_probability).abs() < f64::EPSILON);
} 

#[test]
fn test_probability_with_zero() {
    let bernoulli = Bernoulli { p_int: 0 };
    assert_eq!(bernoulli.p(), 0.0);
}

