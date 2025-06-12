// Answer 0

#[test]
fn test_next_utf8_valid_ascii() {
    let text = b"Hello, World!";
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 1);
}

#[test]
fn test_next_utf8_two_byte_sequence() {
    let text = b"\xC2\xA0"; // U+00A0 (Non-breaking space)
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 2);
}

#[test]
fn test_next_utf8_three_byte_sequence() {
    let text = b"\xE2\x82\xAC"; // U+20AC (Euro sign)
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 3);
}

#[test]
fn test_next_utf8_four_byte_sequence() {
    let text = b"\xF0\x9F\x98\x81"; // U+1F601 (Grinning face with smiling eyes)
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 4);
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text = b"Hello, World!";
    let i = text.len(); // Out-of-bounds index
    let result = next_utf8(text, i);
    assert_eq!(result, text.len() + 1);
}

#[test]
fn test_next_utf8_bound_case() {
    let text = &[0x7F]; // Testing boundary b == 0x7F
    let i = 0;
    let result = next_utf8(text, i);
    assert_eq!(result, 1);
}

