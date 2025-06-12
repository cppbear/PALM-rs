// Answer 0

#[test]
fn test_next_utf8_valid_case_one() {
    let text = [0b1111_1111]; // 255, a 4-byte character start
    let result = next_utf8(&text, 0);
    assert_eq!(result, 1); // inc should be 1
}

#[test]
fn test_next_utf8_valid_case_two() {
    let text = [0b1100_0001, 0b1010_0001]; // First byte is 193 (2-byte character)
    let result = next_utf8(&text, 0);
    assert_eq!(result, 2); // inc should be 2

    let result = next_utf8(&text, 1);
    assert_eq!(result, 2); // Should remain at 2, as byte 1 is continuation
}

#[test]
fn test_next_utf8_valid_case_three() {
    let text = [0b1110_1000, 0b1000_0001, 0b1010_0000]; // First byte is 224 (3-byte character)
    let result = next_utf8(&text, 0);
    assert_eq!(result, 3); // inc should be 3

    let result = next_utf8(&text, 1);
    assert_eq!(result, 3); // Should remain at 3, as byte 1 is continuation

    let result = next_utf8(&text, 2);
    assert_eq!(result, 3); // Should remain at 3, as byte 2 is continuation
}

#[test]
fn test_next_utf8_valid_case_four() {
    let text = [0b1111_0000, 0b1000_0000, 0b1000_0000, 0b1000_0000]; // 4-byte character
    let result = next_utf8(&text, 0);
    assert_eq!(result, 4); // inc should be 4
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text: &[u8] = &[]; // Empty text
    let result = next_utf8(text, 0);
    assert_eq!(result, 1); // Should return 1 since i is out of bounds.
}

