// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let result = starts_with_digit("");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_non_digit_character() {
    let result = starts_with_digit("a");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_space_character() {
    let result = starts_with_digit(" ");
    assert_eq!(result, false);
}

#[test]
fn test_starts_with_digit_special_character() {
    let result = starts_with_digit("#");
    assert_eq!(result, false);
}

