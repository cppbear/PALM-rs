// Answer 0

#[test]
fn test_error_fmt_empty_range() {
    use core::fmt::Formatter;

    let error = Error::EmptyRange;
    let mut buffer = vec![];
    let res = error.fmt(&mut Formatter::new(&mut buffer));
    assert!(res.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "low > high (or equal if exclusive) in uniform distribution");
}

#[test]
fn test_error_fmt_non_finite() {
    use core::fmt::Formatter;

    let error = Error::NonFinite;
    let mut buffer = vec![];
    let res = error.fmt(&mut Formatter::new(&mut buffer));
    assert!(res.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Non-finite range in uniform distribution");
}

