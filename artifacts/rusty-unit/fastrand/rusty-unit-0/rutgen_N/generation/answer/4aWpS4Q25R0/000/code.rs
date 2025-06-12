// Answer 0

#[test]
#[should_panic]
fn test_digit_with_base_zero() {
    digit(0);
}

#[test]
#[should_panic]
fn test_digit_with_base_greater_than_36() {
    digit(37);
}

#[test]
fn test_digit_with_base_10() {
    let result = digit(10);
    assert!(result >= '0' && result <= '9');
}

#[test]
fn test_digit_with_base_36() {
    let result = digit(36);
    assert!(result >= '0' && result <= 'z');
}

#[test]
fn test_digit_with_base_2() {
    let result = digit(2);
    assert!(result == '0' || result == '1');
}

#[test]
fn test_digit_with_base_16() {
    let result = digit(16);
    assert!(result >= '0' && result <= '9' || result >= 'a' && result <= 'f');
}

