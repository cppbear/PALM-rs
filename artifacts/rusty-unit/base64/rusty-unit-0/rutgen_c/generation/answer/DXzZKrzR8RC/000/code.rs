// Answer 0

#[test]
fn test_encode_empty_input() {
    let input: &[u8] = b"";
    let result = encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_single_byte() {
    let input: &[u8] = b"A";
    let result = encode(input);
    assert_eq!(result, "QQ==");
}

#[test]
fn test_encode_multiple_bytes() {
    let input: &[u8] = b"Hello";
    let result = encode(input);
    assert_eq!(result, "SGVsbG8=");
}

#[test]
fn test_encode_exact_block() {
    let input: &[u8] = b"Man";
    let result = encode(input);
    assert_eq!(result, "TWFu");
}

#[test]
fn test_encode_with_padding() {
    let input: &[u8] = b"Any carnal pleasure.";
    let result = encode(input);
    assert_eq!(result, "QW55IGNhcm5hbCBwbGVhc3VyZS4=");
}

