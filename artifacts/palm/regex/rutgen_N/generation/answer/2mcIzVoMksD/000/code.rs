// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let result = escape_bytes(&[]);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte() {
    let result = escape_bytes(&[0]);
    assert_eq!(result, "\\0");
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let result = escape_bytes(&[1, 2, 3]);
    assert_eq!(result, "\\x01\\x02\\x03");
}

#[test]
fn test_escape_bytes_special_bytes() {
    let result = escape_bytes(&[b'a', b'\n', b'\t', b'\\', 255]);
    assert_eq!(result, "a\\n\\t\\\\\\xff");
}

#[test]
fn test_escape_bytes_boundary_value() {
    let result = escape_bytes(&[128, 255]);
    assert_eq!(result, "\\x80\\xff");
}

