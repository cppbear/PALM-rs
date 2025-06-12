// Answer 0

#[test]
fn test_previous_char_empty_input() {
    let input = ByteInput { text: &[], only_utf8: true };
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    let _ = input.previous_char(at);
}

#[test]
fn test_previous_char_single_byte() {
    let input = ByteInput { text: &[0b01100001], only_utf8: true };
    let at = InputAt { pos: 1, c: Char(0), byte: Some(0b01100001), len: 1 };
    let _ = input.previous_char(at);
}

#[test]
fn test_previous_char_two_bytes() {
    let input = ByteInput { text: &[0b11000010, 0b10111100], only_utf8: true };
    let at = InputAt { pos: 2, c: Char(0), byte: Some(0b10111100), len: 2 };
    let _ = input.previous_char(at);
}

#[test]
fn test_previous_char_three_bytes() {
    let input = ByteInput { text: &[0b11100010, 0b10000010, 0b10111100], only_utf8: true };
    let at = InputAt { pos: 3, c: Char(0), byte: Some(0b10111100), len: 3 };
    let _ = input.previous_char(at);
}

#[test]
fn test_previous_char_four_bytes() {
    let input = ByteInput { text: &[0b11110000, 0b10010000, 0b10010000, 0b10000001], only_utf8: true };
    let at = InputAt { pos: 4, c: Char(0), byte: Some(0b10000001), len: 4 };
    let _ = input.previous_char(at);
}

#[should_panic]
#[test]
fn test_previous_char_out_of_bounds() {
    let input = ByteInput { text: &[0b01100001], only_utf8: true };
    let at = InputAt { pos: 2, c: Char(0), byte: None, len: 0 };
    let _ = input.previous_char(at);
}

