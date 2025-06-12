// Answer 0

#[test]
fn test_f64_from_parts_out_of_range_positive() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 1, -1);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_f64_from_parts_out_of_range_negative() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(false, 1, -1);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_f64_from_parts_zero() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 0, -5);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_f64_from_parts_infinite() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 1 << 63, 309);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_f64_from_parts_positive_zero_exponent() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 100, 0);
    assert_eq!(result, Ok(100.0));
}

#[test]
fn test_f64_from_parts_negative_zero_exponent() {
    struct TestStruct;
    
    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange)
        }
    }
    
    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(false, 100, 0);
    assert_eq!(result, Ok(-100.0));
}

