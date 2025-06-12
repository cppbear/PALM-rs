// Answer 0

#[test]
fn test_next_utf8_valid_case() {
    let text: &[u8] = &[0b1110_1111]; // The byte 0b1110_1111 is the maximum valid 3-byte UTF-8 start byte
    let i = 0; // Starting index
    let result = next_utf8(text, i);
    assert_eq!(result, 1); // Inc for 3-byte sequence is 2, so i + inc = 0 + 2 = 1
}

#[test]
fn test_next_utf8_out_of_bound_case() {
    let text: &[u8] = &[0b1110_1111]; // Again, using 0b1110_1111
    let i = 1; // Starting index beyond the size of the text slice
    let result = next_utf8(text, i);
    assert_eq!(result, 2); // Since i is out of bounds, return i + 1 = 1 + 1 = 2
}

#[test]
fn test_next_utf8_boundary_case() {
    let text: &[u8] = &[0b1111_1111]; // Testing with maximum byte, expecting to trigger 4-byte sequence
    let i = 0; // Starting at first byte
    let result = next_utf8(text, i);
    assert_eq!(result, 1); // Inc for an invalid byte will be consistent with the implementation, return i + 1 = 0 + 1 = 1
}

