// Answer 0

#[test]
fn test_error_display_empty_range() {
    let error = Error::EmptyRange;
    let mut buf = String::new();
    let result = error.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "low > high (or equal if exclusive) in uniform distribution");
}

#[test]
fn test_error_display_non_finite() {
    let error = Error::NonFinite;
    let mut buf = String::new();
    let result = error.fmt(&mut buf);
    assert!(result.is_ok());
    assert_eq!(buf, "Non-finite range in uniform distribution");
}

