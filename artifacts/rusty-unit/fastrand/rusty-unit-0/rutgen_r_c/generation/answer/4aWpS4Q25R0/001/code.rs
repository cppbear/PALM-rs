// Answer 0

#[test]
#[should_panic]
fn test_digit_zero_base() {
    digit(0);
}

#[test]
#[should_panic]
fn test_digit_base_greater_than_36() {
    digit(37);
}

#[test]
fn test_digit_valid_base_10() {
    let result = digit(10);
    assert!(result.is_digit(10 as u32));
}

#[test]
fn test_digit_valid_base_16() {
    let result = digit(16);
    assert!(result.is_digit(16 as u32) || result.is_ascii_alphabetic() && result <= 'f');
}

#[test]
fn test_digit_valid_base_36() {
    let result = digit(36);
    assert!(result.is_digit(10 as u32) || (result >= 'a' && result <= 'z'));
}

