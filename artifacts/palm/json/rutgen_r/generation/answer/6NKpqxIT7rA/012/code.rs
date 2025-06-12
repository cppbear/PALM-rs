// Answer 0

#[test]
fn test_f64_from_parts_valid_positive() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let significand: u64 = 1;
    let exponent: i32 = -1; // Makes exponent >= 0 false
    let positive = true;

    let result = mock.f64_from_parts(positive, significand, exponent);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.1); // because 1 * 10^-1 = 0.1
}

#[test]
fn test_f64_from_parts_invalid_negative_exponent() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let significand: u64 = 10_000;
    let exponent: i32 = -10; // Makes exponent >= 0 false
    let positive = true;

    let result = mock.f64_from_parts(positive, significand, exponent);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.0); // because 10000 * 10^-10 = 1.0
}

#[test]
fn test_f64_from_parts_number_out_of_range() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let significand: u64 = 1;
    let exponent: i32 = 309; // Makes exponent >= 0 true, forcing an error
    let positive = false;

    let result = mock.f64_from_parts(positive, significand, exponent);
    assert!(result.is_err());
    assert!(mock.error_called);
}

#[test]
fn test_f64_from_parts_zero_significand() {
    struct Mock {
        error_called: bool,
    }

    impl Mock {
        fn error(&mut self, _: ErrorCode) -> () {
            self.error_called = true;
        }
    }

    let mut mock = Mock { error_called: false };
    let significand: u64 = 0; // f == 0.0 is false
    let exponent: i32 = -5; // Makes exponent >= 0 false
    let positive = true;

    let result = mock.f64_from_parts(positive, significand, exponent);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.0); // Should return 0.0
}

