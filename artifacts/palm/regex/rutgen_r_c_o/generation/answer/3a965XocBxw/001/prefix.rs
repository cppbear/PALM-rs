// Answer 0

#[test]
fn test_empty_array() {
    let bytes: &[u8] = &[];
    char_len_lossy(bytes);
}

#[test]
fn test_valid_utf8_string() {
    let bytes: &[u8] = b"hello world";
    char_len_lossy(bytes);
}

#[test]
fn test_invalid_utf8_sequence() {
    let bytes: &[u8] = &[0x80, 0xFF];
    char_len_lossy(bytes);
}

#[test]
fn test_large_valid_utf8_string() {
    let bytes: &[u8] = b"a".repeat(10_000).as_slice();
    char_len_lossy(bytes);
}

#[test]
fn test_large_invalid_utf8_sequence() {
    let bytes: &[u8] = &[0x80; 10_000];
    char_len_lossy(bytes);
}

