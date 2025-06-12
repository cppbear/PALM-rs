// Answer 0

#[test]
fn test_f64_from_parts_exceeds_positive_range() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 1 << 60, 308);
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

#[test]
fn test_f64_from_parts_exceeds_negative_range() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(false, 1 << 60, -1);
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

#[test]
fn test_f64_from_parts_significand_zero() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 0, -10);
    assert_eq!(result, Ok(0.0));
}

#[test]
fn test_f64_from_parts_infinite_value() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, u64::MAX, 1000);
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

#[test]
fn test_f64_from_parts_valid_value() {
    struct TestStruct;

    impl TestStruct {
        fn error(&self, _: ErrorCode) -> Result<f64> {
            Ok(1.0) // Expected valid output for testing
        }
    }

    let mut test_instance = TestStruct;
    let result = test_instance.f64_from_parts(true, 1, 0);
    assert_eq!(result, Ok(1.0));
}

