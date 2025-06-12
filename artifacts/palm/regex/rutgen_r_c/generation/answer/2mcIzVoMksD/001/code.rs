// Answer 0

#[test]
fn test_escape_bytes_with_normal_bytes() {
    let input: &[u8] = &['a' as u8, 'b' as u8, 'c' as u8];
    let result = escape_bytes(input);
    assert_eq!(result, "abc");
}

#[test]
fn test_escape_bytes_with_control_bytes() {
    let input: &[u8] = &[0, 1, 2, 3];
    let result = escape_bytes(input);
    assert_eq!(result, "\\x00\\x01\\x02\\x03");
}

#[test]
fn test_escape_bytes_with_non_ascii_bytes() {
    let input: &[u8] = &[255, 128, 127];
    let result = escape_bytes(input);
    assert_eq!(result, "\\xff\\x80\\x7f");
}

#[test]
fn test_escape_bytes_with_empty_input() {
    let input: &[u8] = &[];
    let result = escape_bytes(input);
    assert_eq!(result, "");
}

#[test]
fn test_escape_bytes_with_high_value_bytes() {
    let input: &[u8] = &[240, 241, 242];
    let result = escape_bytes(input);
    assert_eq!(result, "\\xf0\\xf1\\xf2");
}

