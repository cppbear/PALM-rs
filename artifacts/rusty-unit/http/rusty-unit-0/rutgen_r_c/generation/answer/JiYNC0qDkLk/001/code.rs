// Answer 0

#[test]
fn test_is_client_error_with_client_error_code() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    assert!(status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_success_code() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    assert!(!status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_redirection_code() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(301) });
    assert!(!status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_server_error_code() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    assert!(!status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_boundary_lower() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(400) });
    assert!(status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_boundary_upper() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(499) });
    assert!(status_code.is_client_error());
}

#[test]
fn test_is_client_error_with_out_of_range_code() {
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(399) });
    assert!(!status_code.is_client_error());
}

