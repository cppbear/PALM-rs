// Answer 0

#[test]
fn test_decode_utf8_single_byte_ascii() {
    let input = [0x00];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_single_byte_max() {
    let input = [0x7F];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_multiple_bytes_invalid() {
    let input = [0xC2, 0x00];
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_empty() {
    let input: &[u8] = &[];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_two_bytes_invalid() {
    let input = [0xC0];
    decode_utf8(&input);
}

