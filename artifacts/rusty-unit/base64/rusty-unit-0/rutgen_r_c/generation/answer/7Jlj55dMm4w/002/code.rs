// Answer 0

#[test]
fn test_encoded_len_with_padding() {
    // Input length that ensures complete_input_chunks.checked_mul(4) does not panic
    let input_length = 10; // 10 / 3 = 3 complete chunks, results in 12,
    let padding = true; // should add 4 padding bytes
    assert_eq!(encoded_len(input_length, padding), Some(16)); // expected 16 bytes

    // Check boundary where complete_input_chunks is just entering into a new value
    let input_length = 3; // 3 / 3 = 1 complete chunk, results in 4,
    let padding = true; // should add 4 padding bytes
    assert_eq!(encoded_len(input_length, padding), Some(8)); // expected 8 bytes

    // Input length that results in a remainder of 1 with padding
    let input_length = 7; // 7 / 3 = 2 complete chunks, results in 8,
    let padding = true; // should add 4 padding bytes
    assert_eq!(encoded_len(input_length, padding), Some(12)); // expected 12 bytes

    // Input length that results in a remainder of 2 with padding
    let input_length = 8; // 8 / 3 = 2 complete chunks, results in 8,
    let padding = true; // should add 4 padding bytes
    assert_eq!(encoded_len(input_length, padding), Some(12)); // expected 12 bytes

    // Maximum safe value for encoding length
    let input_length = usize::MAX - 1; // should (http://isize.MAX / 3) would produce overflow in output
    let padding = true;
    assert_eq!(encoded_len(input_length, padding), None); // expected None due to overflow
}

