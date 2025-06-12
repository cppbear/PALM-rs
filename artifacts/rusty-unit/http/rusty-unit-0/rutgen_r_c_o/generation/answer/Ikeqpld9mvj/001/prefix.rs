// Answer 0

#[test]
fn test_is_server_error_500() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_599() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(599) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_501() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(501) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_549() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(549) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_600() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(600) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_500_panic() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_599_panic() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(599) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_out_of_bounds() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(450) });
    status.is_server_error();
}

#[test]
fn test_is_server_error_negative_value_panic() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    status.is_server_error();
}

