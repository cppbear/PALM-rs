// Answer 0

#[test]
fn test_is_informational_with_informational_code() {
    struct TestStatusCode(u16);
    
    impl TestStatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(NonZeroU16::new(code).unwrap())
        }
    }

    let status = TestStatusCode::new(100);
    assert!(status.0.is_informational());
}

#[test]
fn test_is_informational_with_non_informational_code() {
    struct TestStatusCode(u16);
    
    impl TestStatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(NonZeroU16::new(code).unwrap())
        }
    }

    let status = TestStatusCode::new(200);
    assert!(!status.0.is_informational());
}

#[test]
fn test_is_informational_with_boundary_codes() {
    struct TestStatusCode(u16);
    
    impl TestStatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(NonZeroU16::new(code).unwrap())
        }
    }

    let lower_boundary = TestStatusCode::new(100);
    let upper_boundary = TestStatusCode::new(199);
    
    assert!(lower_boundary.0.is_informational());
    assert!(upper_boundary.0.is_informational());
}

