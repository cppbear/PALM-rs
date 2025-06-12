// Answer 0

#[test]
fn test_next_utf8_case_one() {
    let text = &[0b110_11111]; // boundary value for 2-byte UTF-8 character
    let i = 0;
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_case_two() {
    let text = &[0b110_11111, 0x00]; // two valid bytes
    let i = 0;
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_case_three() {
    let text = &[0b110_11111, 0b00000001]; // two valid bytes
    let i = 0;
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_case_four() {
    let text = &[0b110_11111, 0b11111111, 0x00]; // three total bytes, still valid
    let i = 0;
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_edge_case() {
    let text = &[0b110_11111, 0b01011010]; // valid continuation byte
    let i = 0;
    next_utf8(text, i);
}

