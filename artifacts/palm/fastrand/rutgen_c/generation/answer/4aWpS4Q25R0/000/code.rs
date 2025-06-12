// Answer 0

#[test]
fn test_digit_valid_range() {
    // Test digit generation with valid bases
    for base in 2..=36 {
        let result = digit(base);
        assert!(result.is_ascii_alphanumeric());
        assert!(result.to_digit(36).unwrap() < base);
    }
}

#[test]
#[should_panic(expected = "Panics if the base is zero or greater than 36.")]
fn test_digit_zero_base() {
    digit(0);
}

#[test]
#[should_panic(expected = "Panics if the base is zero or greater than 36.")]
fn test_digit_base_above_limit() {
    digit(37);
}

