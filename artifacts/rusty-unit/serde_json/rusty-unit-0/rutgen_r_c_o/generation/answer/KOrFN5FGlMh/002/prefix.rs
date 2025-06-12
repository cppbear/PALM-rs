// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let slice = "";
    let result = starts_with_digit(slice);
}

#[test]
fn test_starts_with_digit_non_digit_char() {
    let slice = "A";
    let result = starts_with_digit(slice);
}

