// Answer 0

#[test]
fn test_uniform_new_inclusive_valid_range() {
    struct SampleType {
        value: f64,
    }

    impl SampleBorrow<f64> for SampleType {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let low = SampleType { value: 1.0 };
    let high = SampleType { value: 10.0 };

    let result = new_inclusive(low, high);
    assert!(result.is_ok());
}

#[test]
fn test_uniform_new_inclusive_invalid_range_low_greater_than_high() {
    struct SampleType {
        value: f64,
    }

    impl SampleBorrow<f64> for SampleType {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let low = SampleType { value: 10.0 };
    let high = SampleType { value: 1.0 };

    let result = new_inclusive(low, high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_non_finite_low() {
    struct SampleType {
        value: f64,
    }

    impl SampleBorrow<f64> for SampleType {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let low = SampleType { value: f64::NAN };
    let high = SampleType { value: 10.0 };

    let result = new_inclusive(low, high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_non_finite_high() {
    struct SampleType {
        value: f64,
    }

    impl SampleBorrow<f64> for SampleType {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let low = SampleType { value: 1.0 };
    let high = SampleType { value: f64::INFINITY };

    let result = new_inclusive(low, high);
    assert!(result.is_err());
}

#[test]
fn test_uniform_new_inclusive_zero_range() {
    struct SampleType {
        value: f64,
    }

    impl SampleBorrow<f64> for SampleType {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let low = SampleType { value: 5.0 };
    let high = SampleType { value: 5.0 };

    let result = new_inclusive(low, high);
    assert!(result.is_ok());
}

