// Answer 0

#[test]
fn test_is_server_error_valid() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        pub fn new(code: u16) -> Result<StatusCode, &'static str> {
            if code >= 100 && code <= 599 {
                Ok(StatusCode(NonZeroU16::new(code).unwrap()))
            } else {
                Err("Invalid Status Code")
            }
        }
    }

    let status_code_500 = TestStatusCode::new(500).unwrap();
    assert!(status_code_500.is_server_error());

    let status_code_501 = TestStatusCode::new(501).unwrap();
    assert!(status_code_501.is_server_error());

    let status_code_599 = TestStatusCode::new(599).unwrap();
    assert!(status_code_599.is_server_error());
}

#[test]
fn test_is_server_error_non_server_error() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        pub fn new(code: u16) -> Result<StatusCode, &'static str> {
            if code >= 100 && code <= 599 {
                Ok(StatusCode(NonZeroU16::new(code).unwrap()))
            } else {
                Err("Invalid Status Code")
            }
        }
    }

    let status_code_200 = TestStatusCode::new(200).unwrap();
    assert!(!status_code_200.is_server_error());

    let status_code_404 = TestStatusCode::new(404).unwrap();
    assert!(!status_code_404.is_server_error());

    let status_code_499 = TestStatusCode::new(499).unwrap();
    assert!(!status_code_499.is_server_error());

    let status_code_600 = TestStatusCode::new(600).unwrap_err();
    // Expect panic or handle invalid code case if necessary
}

#[test]
#[should_panic]
fn test_is_server_error_invalid_code() {
    struct TestStatusCode(u16);
    impl TestStatusCode {
        pub fn new(code: u16) -> Result<StatusCode, &'static str> {
            if code >= 100 && code <= 599 {
                Ok(StatusCode(NonZeroU16::new(code).unwrap()))
            } else {
                panic!("Invalid Status Code: {}", code);
            }
        }
    }

    let _ = TestStatusCode::new(600).unwrap(); // should panic
}

