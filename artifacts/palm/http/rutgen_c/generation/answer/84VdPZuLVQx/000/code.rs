// Answer 0

#[test]
fn test_is_redirection_successful() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(NonZeroU16::new(code).unwrap())
        }
    }

    // Test cases for 300 to 399 range
    let test_codes = vec![
        TestStatusCode::new(300),
        TestStatusCode::new(301),
        TestStatusCode::new(350),
        TestStatusCode::new(399),
    ];

    for code in test_codes {
        assert!(code.is_redirection());
    }
}

#[test]
fn test_is_redirection_failure() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(NonZeroU16::new(code).unwrap())
        }
    }

    // Test cases outside the 300 to 399 range
    let test_codes = vec![
        TestStatusCode::new(200), // OK
        TestStatusCode::new(400), // Bad Request
        TestStatusCode::new(500), // Internal Server Error
        TestStatusCode::new(299), // Just below 300
    ];

    for code in test_codes {
        assert!(!code.is_redirection());
    }
}

