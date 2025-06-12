// Answer 0

#[test]
fn test_decode_utf8_case1() {
    let input = [0b11110_001, 0x80, 0x80, 0x80]; // Invalid codepoint range (0x110000)
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_case2() {
    let input = [0b11110_010, 0xBF, 0xBF, 0xBF]; // Invalid codepoint range (0x110001)
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_case3() {
    let input = [0b11110_011, 0x80, 0x80, 0xBF]; // Invalid codepoint range (0x110002)
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_case4() {
    let input = [0b11110_100, 0x9F, 0xBF, 0xBF]; // Invalid codepoint range (0x110003)
    decode_utf8(&input);
}

#[test]
fn test_decode_utf8_case5() {
    let input = [0b11110_101, 0x80, 0x80, 0xBF]; // Invalid codepoint range (0x110004)
    decode_utf8(&input);
}

