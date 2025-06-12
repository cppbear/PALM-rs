// Answer 0

#[test]
fn test_encoded_len_rem_1_no_padding() {
    // Test input where bytes_len % 3 == 1 and padding is false
    let bytes_len = 1; // 1 % 3 == 1
    let padding = false;

    let result = encoded_len(bytes_len, padding);
    assert_eq!(result, Some(2)); // Expecting encoded length when padding is false
}

#[test]
fn test_encoded_len_rem_1_no_padding_large_input() {
    // Test input with a large value that should not overflow
    let bytes_len = 1 + 3 * 100; // 301 % 3 == 1
    let padding = false;

    let result = encoded_len(bytes_len, padding);
    assert_eq!(result, Some(402)); // 100 complete chunks * 4 + 2 for remainder 1
}

