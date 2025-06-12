// Answer 0

#[test]
fn test_probability_with_valid_input() {
    struct Bernoulli {
        p_int: usize,
    }

    const ALWAYS_TRUE: usize = 1;
    const SCALE: usize = 10;

    let distribution = Bernoulli { p_int: 5 }; // p_int is not ALWAYS_TRUE
    let result = distribution.p();
    assert!((result - 0.5).abs() < f64::EPSILON); // Expecting (5.0 / 10.0) = 0.5
}

#[test]
fn test_probability_with_zero_input() {
    struct Bernoulli {
        p_int: usize,
    }

    const ALWAYS_TRUE: usize = 1;
    const SCALE: usize = 10;

    let distribution = Bernoulli { p_int: 0 }; // p_int is not ALWAYS_TRUE
    let result = distribution.p();
    assert_eq!(result, 0.0); // Expecting (0.0 / 10.0) = 0.0
}

#[test]
fn test_probability_with_maximum_input() {
    struct Bernoulli {
        p_int: usize,
    }

    const ALWAYS_TRUE: usize = 1;
    const SCALE: usize = 10;

    let distribution = Bernoulli { p_int: 10 }; // p_int is not ALWAYS_TRUE
    let result = distribution.p();
    assert!((result - 1.0).abs() < f64::EPSILON); // Expecting (10.0 / 10.0) = 1.0
}

#[test]
fn test_probability_with_one_less_than_scale() {
    struct Bernoulli {
        p_int: usize,
    }

    const ALWAYS_TRUE: usize = 1;
    const SCALE: usize = 10;

    let distribution = Bernoulli { p_int: 9 }; // p_int is not ALWAYS_TRUE
    let result = distribution.p();
    assert!((result - 0.9).abs() < f64::EPSILON); // Expecting (9.0 / 10.0) = 0.9
}

