// Answer 0

#[test]
fn test_uniform_new_inclusive_valid_range() {
    use rand::uniform::{Uniform, SampleBorrow};

    struct TestRange {
        low: f64,
        high: f64,
    }

    let valid_range = TestRange { low: 1.0, high: 10.0 };
    let result = Uniform::new_inclusive(valid_range.low, valid_range.high);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_inclusive_low_greater_than_high() {
    use rand::uniform::{Uniform, SampleBorrow};

    struct TestRange {
        low: f64,
        high: f64,
    }

    let invalid_range = TestRange { low: 10.0, high: 1.0 };
    let result = Uniform::new_inclusive(invalid_range.low, invalid_range.high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_low_equals_high() {
    use rand::uniform::{Uniform, SampleBorrow};

    struct TestRange {
        low: f64,
        high: f64,
    }

    let equal_range = TestRange { low: 5.0, high: 5.0 };
    let result = Uniform::new_inclusive(equal_range.low, equal_range.high);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_inclusive_non_finite_low() {
    use rand::uniform::{Uniform, SampleBorrow};
    use std::f64;

    struct TestRange {
        low: f64,
        high: f64,
    }

    let non_finite_range = TestRange {
        low: f64::NAN,
        high: 10.0,
    };
    let result = Uniform::new_inclusive(non_finite_range.low, non_finite_range.high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_non_finite_high() {
    use rand::uniform::{Uniform, SampleBorrow};
    use std::f64;

    struct TestRange {
        low: f64,
        high: f64,
    }

    let non_finite_range = TestRange {
        low: 1.0,
        high: f64::INFINITY,
    };
    let result = Uniform::new_inclusive(non_finite_range.low, non_finite_range.high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_non_finite_range() {
    use rand::uniform::{Uniform, SampleBorrow};
    use std::f64;

    struct TestRange {
        low: f64,
        high: f64,
    }

    let non_finite_range = TestRange {
        low: f64::NEG_INFINITY,
        high: f64::INFINITY,
    };
    let result = Uniform::new_inclusive(non_finite_range.low, non_finite_range.high);
    assert!(result.is_err());
}

