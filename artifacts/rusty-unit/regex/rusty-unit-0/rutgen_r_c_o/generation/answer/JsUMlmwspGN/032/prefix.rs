// Answer 0

#[test]
fn test_decode_utf8_case1() {
    let input: &[u8] = &[0b1110_0000, 0b10_000000, 0x00];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_case2() {
    let input: &[u8] = &[0b1110_0001, 0x00, 0xFF];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_case3() {
    let input: &[u8] = &[0b1110_1111, 0b00_111111, 0x80];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_case4() {
    let input: &[u8] = &[0b1110_0010, 0xB0, 0xBF];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_case5() {
    let input: &[u8] = &[0b1110_1000, 0xB0, 0x40];
    decode_utf8(input);
}

#[test]
fn test_decode_utf8_case6() {
    let input: &[u8] = &[0b1110_0100, 0x80, 0b100000];
    decode_utf8(input);
}

