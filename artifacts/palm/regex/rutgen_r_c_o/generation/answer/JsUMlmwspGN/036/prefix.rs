// Answer 0

#[test]
fn test_decode_utf8_invalid_sequence() {
    let src: &[u8] = &[0b1110_0010, 0b1000_0000, 0b1000_0000];
    decode_utf8(src);
}

