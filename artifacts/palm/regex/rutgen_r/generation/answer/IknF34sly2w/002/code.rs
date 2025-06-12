// Answer 0

#[test]
fn test_escape_bytes_empty() {
    let result = escape_bytes(&[]);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_single_byte() {
    let result = escape_bytes(&[b'a']);
    assert_eq!(result, escape_byte(b'a'));
}

#[test]
fn test_escape_bytes_multiple_bytes() {
    let result = escape_bytes(&[b'a', b'b', b'c']);
    assert_eq!(result, escape_byte(b'a') + &escape_byte(b'b') + &escape_byte(b'c'));
}

#[test]
fn test_escape_bytes_special_characters() {
    let result = escape_bytes(&[b'\n', b'\t', b'\r']);
    assert_eq!(result, escape_byte(b'\n') + &escape_byte(b'\t') + &escape_byte(b'\r'));
}

#[test]
fn test_escape_bytes_boundary_conditions() {
    let result = escape_bytes(&[0, 255]);
    assert_eq!(result, escape_byte(0) + &escape_byte(255));
}

