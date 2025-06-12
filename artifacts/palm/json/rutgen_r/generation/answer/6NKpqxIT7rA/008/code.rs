// Answer 0

#[test]
fn test_f64_from_parts_number_out_of_range_positive() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _code: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;

    let positive = true;
    let significand = 1; // valid significand, must not be zero
    let exponent = 0; // satisfies exponent >= 0
    
    let result = test_instance.f64_from_parts(positive, significand, exponent);

    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

#[test]
fn test_f64_from_parts_number_out_of_range_negative() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _code: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;

    let positive = false; 
    let significand = 1; // valid significand, must not be zero
    let exponent = 0; // satisfies exponent >= 0

    let result = test_instance.f64_from_parts(positive, significand, exponent);

    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

