// Answer 0

#[test]
fn test_is_client_error_below_range() {
    let status = StatusCode(NonZeroU16::new(300).unwrap());
    status.is_client_error();
}

#[test]
fn test_is_client_error_exactly_400() {
    let status = StatusCode(NonZeroU16::new(400).unwrap());
    status.is_client_error();
}

#[test]
fn test_is_client_error_within_range() {
    let status = StatusCode(NonZeroU16::new(450).unwrap());
    status.is_client_error();
}

#[test]
fn test_is_client_error_exactly_499() {
    let status = StatusCode(NonZeroU16::new(499).unwrap());
    status.is_client_error();
}

#[test]
fn test_is_client_error_above_range() {
    let status = StatusCode(NonZeroU16::new(500).unwrap());
    status.is_client_error();
}

#[test]
fn test_is_client_error_high_valid_value() {
    let status = StatusCode(NonZeroU16::new(65535).unwrap());
    status.is_client_error();
}

