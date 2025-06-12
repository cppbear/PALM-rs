// Answer 0

#[test]
fn test_uniform_new_success_integers() {
    use std::ops::Range;
    struct IntegerSampler;
    
    impl SampleBorrow<i32> for IntegerSampler {
        // Sample implementation omitted for brevity
    }

    let low = 0;
    let high = 10;
    let result = new(low, high);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_success_floats() {
    use std::ops::Range;
    struct FloatSampler;

    impl SampleBorrow<f32> for FloatSampler {
        // Sample implementation omitted for brevity
    }

    let low = 0.0;
    let high = 1.0;
    let result = new(low, high);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_uniform_new_failure_low_ge_high() {
    let low = 10;
    let high = 10;
    let _ = new(low, high);
}

#[test]
#[should_panic]
fn test_uniform_new_failure_non_finite_low() {
    let low = f32::NAN;
    let high = 10.0;
    let _ = new(low, high);
}

#[test]
#[should_panic]
fn test_uniform_new_failure_non_finite_high() {
    let low = 0.0;
    let high = f32::INFINITY;
    let _ = new(low, high);
}

#[test]
#[should_panic]
fn test_uniform_new_failure_non_finite_range() {
    let low = f64::NEG_INFINITY;
    let high = f64::INFINITY;
    let _ = new(low, high);
}

