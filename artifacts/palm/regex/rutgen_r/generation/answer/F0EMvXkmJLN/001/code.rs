// Answer 0

#[test]
fn test_is_start_byte() {
    // Test with a byte that is a valid start byte (110xxxxx)
    let valid_start_byte_1: u8 = 0b11000000; // Should return true
    assert!(is_start_byte(valid_start_byte_1));

    // Test with a byte that is a valid start byte (1110xxxx)
    let valid_start_byte_2: u8 = 0b11100000; // Should return true
    assert!(is_start_byte(valid_start_byte_2));

    // Test with a byte that is not a valid start byte (10xxxxxx)
    let invalid_start_byte_1: u8 = 0b10000000; // Should return false
    assert!(!is_start_byte(invalid_start_byte_1));

    // Test with a byte that is not a valid start byte (10xxxxxx)
    let invalid_start_byte_2: u8 = 0b10111111; // Should return false
    assert!(!is_start_byte(invalid_start_byte_2));

    // Test with a byte that is not a valid start byte (0xxxxxxx)
    let invalid_start_byte_3: u8 = 0b00000000; // Should return false
    assert!(!is_start_byte(invalid_start_byte_3));

    // Test with a byte that is not a valid start byte (1111xxxx)
    let invalid_start_byte_4: u8 = 0b11110000; // Should return true, but is not a valid start byte as per the UTF-8 rules.
    assert!(is_start_byte(invalid_start_byte_4));
}

