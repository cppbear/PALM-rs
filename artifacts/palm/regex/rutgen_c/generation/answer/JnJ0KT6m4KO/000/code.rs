// Answer 0

#[test]
fn test_freq_rank_valid_range() {
    let byte: u8 = 0; // Test with the minimum valid range
    let expected = BYTE_FREQUENCIES[0] as usize;
    let result = freq_rank(byte);
    assert_eq!(result, expected);
}

#[test]
fn test_freq_rank_upper_bound() {
    let byte: u8 = 255; // Test with the maximum valid range
    let expected = BYTE_FREQUENCIES[255] as usize;
    let result = freq_rank(byte);
    assert_eq!(result, expected);
}

#[test]
fn test_freq_rank_mid_range() {
    let byte: u8 = 127; // Test with a mid-range value
    let expected = BYTE_FREQUENCIES[127] as usize;
    let result = freq_rank(byte);
    assert_eq!(result, expected);
}

