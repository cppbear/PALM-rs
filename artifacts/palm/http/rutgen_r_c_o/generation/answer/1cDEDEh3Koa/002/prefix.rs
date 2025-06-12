// Answer 0

#[test]
fn test_from_bytes_zero() {
    let src = [b'0', b'0', b'0'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_valid_100() {
    let src = [b'1', b'0', b'0'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_valid_507() {
    let src = [b'5', b'0', b'7'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_valid_999() {
    let src = [b'9', b'9', b'9'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_invalid_zero_prefix() {
    let src = [b'0', b'1', b'0'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_invalid_too_large() {
    let src = [b'1', b'1', b'0'];
    from_bytes(&src);
}

#[test]
fn test_from_bytes_invalid_not_digits() {
    let src = [b'A', b'1', b'0'];
    from_bytes(&src);
}

