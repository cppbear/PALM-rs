// Answer 0

#[test]
fn test_new_invalid_weight_negative() {
    // Struct to satisfy the SampleBorrow and Weight traits
    struct WeightStruct {
        value: f64,
    }

    impl Weight for WeightStruct {
        const ZERO: Self = WeightStruct { value: 0.0 };
    }

    impl SampleBorrow<f64> for WeightStruct {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let weights = vec![WeightStruct { value: -1.0 }];
    let result = new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_invalid_weight_nan() {
    // Struct to satisfy the SampleBorrow and Weight traits
    struct WeightStruct {
        value: f64,
    }

    impl Weight for WeightStruct {
        const ZERO: Self = WeightStruct { value: 0.0 };
    }

    impl SampleBorrow<f64> for WeightStruct {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let weights = vec![WeightStruct { value: f64::NAN }];
    let result = new(weights);
    assert_eq!(result, Err(Error::InvalidWeight));
}

#[test]
fn test_new_insufficient_nonzero() {
    // Struct to satisfy the SampleBorrow and Weight traits
    struct WeightStruct {
        value: f64,
    }

    impl Weight for WeightStruct {
        const ZERO: Self = WeightStruct { value: 0.0 };
    }

    impl SampleBorrow<f64> for WeightStruct {
        fn borrow(&self) -> &f64 {
            &self.value
        }
    }

    let weights = vec![WeightStruct { value: 0.0 }, WeightStruct { value: 0.0 }];
    let result = new(weights);
    assert_eq!(result, Err(Error::InsufficientNonZero));
}

