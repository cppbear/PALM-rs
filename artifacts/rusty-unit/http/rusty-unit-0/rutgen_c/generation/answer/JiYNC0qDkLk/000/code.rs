// Answer 0

#[test]
fn test_is_client_error_with_client_error_code() {
    struct TestStatusCode(u16);
    impl StatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(unsafe { NonZeroU16::new_unchecked(code) })
        }
    }
    let status_code = StatusCode::new(404); // Client Error
    assert!(status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_non_client_error_code() {
    struct TestStatusCode(u16);
    impl StatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(unsafe { NonZeroU16::new_unchecked(code) })
        }
    }
    let status_code = StatusCode::new(200); // Success
    assert!(!status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_boundary_values() {
    struct TestStatusCode(u16);
    impl StatusCode {
        fn new(code: u16) -> StatusCode {
            StatusCode(unsafe { NonZeroU16::new_unchecked(code) })
        }
    }
    let status_code_below = StatusCode::new(399); // Below range
    assert!(!status_code_below.is_client_error());

    let status_code_boundary = StatusCode::new(400); // Lower boundary of client errors
    assert!(status_code_boundary.is_client_error());

    let status_code_upper_boundary = StatusCode::new(499); // Upper boundary of client errors
    assert!(status_code_upper_boundary.is_client_error());

    let status_code_above = StatusCode::new(500); // Above range
    assert!(!status_code_above.is_client_error());
}

