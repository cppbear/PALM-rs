// Answer 0

#[test]
fn test_decode_utf8_three_bytes_invalid_b2() {
    let src: Vec<u8> = vec![0b1110_0000, 0b1000_0000, 0b1100_0000];
    let result = decode_utf8(&src);
}

#[test]
fn test_decode_utf8_three_bytes_correct_b1_b2() {
    let src: Vec<u8> = vec![0b1110_0000, 0b1000_0000, 0b1000_0000];
    let result = decode_utf8(&src);
}

#[test]
fn test_decode_utf8_three_bytes_short() {
    let src: Vec<u8> = vec![0b1110_0000, 0b1000_0000];
    let result = decode_utf8(&src);
}

#[test]
fn test_decode_utf8_three_bytes_correct_valid() {
    let src: Vec<u8> = vec![0b1110_0001, 0b1000_0000, 0b1000_0000];
    let result = decode_utf8(&src);
}

#[test]
fn test_decode_utf8_three_bytes_invalid_first() {
    let src: Vec<u8> = vec![0b1110_1111, 0b1000_0000, 0b1000_0000];
    let result = decode_utf8(&src);
}

