// Answer 0

#[test]
fn test_f64_from_parts_exponent_out_of_range() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let positive = false;
    let significand = 0; // f will be 0.0
    let exponent = -1; // will trigger the None condition in POW10

    let result = f64_from_parts(&mut mock, positive, significand, exponent);

    assert_eq!(result, Ok(0.0));
    assert!(!mock.error_called);
}

#[test]
fn test_f64_from_parts_exponent_negative_large() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let positive = false;
    let significand = 1;
    let exponent = -309; // Out of range for POW10, should lead to NumberOutOfRange

    let result = f64_from_parts(&mut mock, positive, significand, exponent);

    assert!(result.is_err());
    assert!(mock.error_called);
}

#[test]
fn test_f64_from_parts_return_negative_zero() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let positive = false;
    let significand = 0; // f will be zero
    let exponent = 0; // valid exponent

    let result = f64_from_parts(&mut mock, positive, significand, exponent);

    assert_eq!(result, Ok(0.0));
    assert!(!mock.error_called);
}

