// Answer 0

#[test]
fn test_sign_extend_zero_bytes() {
    let val = 0u64;
    let nbytes = 0;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, 0);
}

#[test]
fn test_sign_extend_full_bytes() {
    let val = 0xFFFFFFFFFFFFFFFFu64;
    let nbytes = 8;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_partial_bytes() {
    let val = 0xFFFFFFFFFFFFFFFFu64;
    let nbytes = 4;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -1);
}

#[test]
fn test_sign_extend_negative_sign() {
    let val = 0x8000000000000000u64;
    let nbytes = 8;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, -9223372036854775808);
}

#[test]
fn test_sign_extend_positive_sign() {
    let val = 0x7FFFFFFFFFFFFFFFu64;
    let nbytes = 8;
    let result = sign_extend(val, nbytes);
    assert_eq!(result, 9223372036854775807);
}

