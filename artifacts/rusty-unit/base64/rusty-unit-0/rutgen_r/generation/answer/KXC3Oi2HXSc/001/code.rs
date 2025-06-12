// Answer 0

#[test]
fn test_new_with_multiple_of_four() {
    let encoded_len = 8; // multiple of 4
    let result = new(encoded_len);
    assert_eq!(result.rem, 0);
    assert_eq!(result.conservative_decoded_len, 6); // (8 / 4) * 3
}

#[test]
fn test_new_with_one_more_than_multiple_of_four() {
    let encoded_len = 9; // 1 more than multiple of 4
    let result = new(encoded_len);
    assert_eq!(result.rem, 1);
    assert_eq!(result.conservative_decoded_len, 9); // (9 / 4 + 1) * 3
}

#[test]
fn test_new_with_two_more_than_multiple_of_four() {
    let encoded_len = 10; // 2 more than multiple of 4
    let result = new(encoded_len);
    assert_eq!(result.rem, 2);
    assert_eq!(result.conservative_decoded_len, 9); // (10 / 4 + 1) * 3
}

#[test]
fn test_new_with_three_more_than_multiple_of_four() {
    let encoded_len = 11; // 3 more than multiple of 4
    let result = new(encoded_len);
    assert_eq!(result.rem, 3);
    assert_eq!(result.conservative_decoded_len, 12); // (11 / 4 + 1) * 3
}

#[test]
fn test_new_with_zero() {
    let encoded_len = 0; // edge case: zero input
    let result = new(encoded_len);
    assert_eq!(result.rem, 0);
    assert_eq!(result.conservative_decoded_len, 0); // (0 / 4) * 3
}

#[test]
fn test_new_with_large_value() {
    let encoded_len = 100000; // a large input
    let result = new(encoded_len);
    assert_eq!(result.rem, 0);
    assert_eq!(result.conservative_decoded_len, 75000); // (100000 / 4) * 3
}

