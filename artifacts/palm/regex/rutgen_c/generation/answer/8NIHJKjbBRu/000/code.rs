// Answer 0

#[test]
fn test_next_utf8_ascii() {
    let text = b"hello"; // ASCII bytes
    let start_index = 0;
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, 1);
}

#[test]
fn test_next_utf8_two_byte() {
    let text = b"\xC2\xA9"; // © (U+00A9) in UTF-8
    let start_index = 0;
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, 2);
}

#[test]
fn test_next_utf8_three_byte() {
    let text = b"\xE2\x9C\x94"; // ✔ (U+2714) in UTF-8
    let start_index = 0;
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, 3);
}

#[test]
fn test_next_utf8_four_byte() {
    let text = b"\xF0\x9F\x8C\x8D"; // 🌍 (U+1F30D) in UTF-8
    let start_index = 0;
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, 4);
}

#[test]
fn test_next_utf8_index_out_of_bounds() {
    let text = b"hello"; // ASCII bytes
    let start_index = text.len(); // Out of bounds
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, text.len() + 1);
}

#[test]
fn test_next_utf8_boundary_case() {
    let text = b"\xC2"; // Incomplete two-byte sequence
    let start_index = 0;
    let next_index = next_utf8(text, start_index);
    assert_eq!(next_index, 1); // Should skip to next byte
}

