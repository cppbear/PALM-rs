// Answer 0

#[test]
fn test_is_valid_with_minimum_valid_input() {
    let b: u8 = 32;
    is_valid(b);
}

#[test]
fn test_is_valid_with_maximum_invalid_input() {
    let b: u8 = 127;
    is_valid(b);
}

#[test]
fn test_is_valid_with_tab_character() {
    let b: u8 = 9;
    is_valid(b);
}

