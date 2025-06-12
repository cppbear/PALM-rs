// Answer 0

#[test]
fn test_previous_char_valid_case() {
    let input = CharInput(b"hello");
    let at = InputAt { pos: 5, c: Char(0), byte: None, len: 5 };
    let result = input.previous_char(at);
    assert_eq!(result, Char('o' as u32)); // Assuming 'o' is the last character
}

#[test]
fn test_previous_char_empty_case() {
    let input = CharInput(b"");
    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    assert!(std::panic::catch_unwind(|| {
        input.previous_char(at);
    }).is_err()); // Expect panic due to invalid index
}

#[test]
fn test_previous_char_single_char() {
    let input = CharInput(b"a");
    let at = InputAt { pos: 1, c: Char(0), byte: None, len: 1 };
    let result = input.previous_char(at);
    assert_eq!(result, Char('a' as u32)); // Expecting 'a'
}

#[test]
fn test_previous_char_multi_byte_character() {
    let input = CharInput(b"\xe2\x9c\x94"); // Check for a multi-byte character (✓)
    let at = InputAt { pos: 3, c: Char(0), byte: None, len: 3 };
    let result = input.previous_char(at);
    assert_eq!(result, Char('✓' as u32)); // Expecting ✓
}

#[test]
fn test_previous_char_out_of_bounds() {
    let input = CharInput(b"abc");
    let at = InputAt { pos: 4, c: Char(0), byte: None, len: 3 };
    assert!(std::panic::catch_unwind(|| {
        input.previous_char(at);
    }).is_err()); // Expect panic due to index out of bounds
}

