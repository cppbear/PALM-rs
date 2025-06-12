// Answer 0

#[test]
fn test_write_checked_success() {
    let src: &[u8] = &[1, 2, 3]; // Assuming METHOD_CHARS[1] = some valid method char, likewise for 2, and 3
    let mut dst: [u8; 3] = [0; 3];
    let result = write_checked(src, &mut dst);
    assert_eq!(result, Ok(()));
    assert_eq!(dst, [METHOD_CHARS[1], METHOD_CHARS[2], METHOD_CHARS[3]]);
}

#[test]
fn test_write_checked_invalid_method_char() {
    let src: &[u8] = &[0]; // Assuming METHOD_CHARS[0] is invalid
    let mut dst: [u8; 1] = [0];
    let result = write_checked(src, &mut dst);
    assert!(result.is_err());
}

#[test]
fn test_write_checked_dst_too_small() {
    let src: &[u8] = &[1, 2]; // Assuming this has valid method chars
    let mut dst: [u8; 1] = [0]; // Not enough space in dst
    let result = write_checked(src, &mut dst);
    assert!(result.is_err());
}

