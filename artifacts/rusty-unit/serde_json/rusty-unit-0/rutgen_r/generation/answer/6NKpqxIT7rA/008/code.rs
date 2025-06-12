// Answer 0

#[test]
fn test_f64_from_parts_number_out_of_range() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }

    let mut instance = TestStruct;
    let positive = true;
    let significand = 1; // non-zero to satisfy f == 0.0 is false
    let exponent = 0; // satisfies exponent >= 0 is true

    let result = instance.f64_from_parts(positive, significand, exponent);
    
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange));
}

#[test]
fn test_f64_from_parts_negative_exponent() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }

    let mut instance = TestStruct;
    let positive = true;
    let significand = 1; // non-zero value
    let exponent = -309; // less than 0 to trigger the boundary condition

    let result = instance.f64_from_parts(positive, significand, exponent);
    
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange));
}

