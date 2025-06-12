// Answer 0

#[test]
fn test_is_valid_with_tab() {
    let input: u8 = b'\t'; // corresponds to the ASCII value of tab
    assert_eq!(is_valid(input), true);
}

#[test]
fn test_is_valid_below_32() {
    let input: u8 = 31; // below the minimum valid range (32)
    assert_eq!(is_valid(input), false);
}

#[test]
fn test_is_valid_equality_condition() {
    let input: u8 = 127; // equals the value that should return false
    assert_eq!(is_valid(input), false);
}

#[test]
fn test_is_valid_above_32() {
    let input: u8 = 32; // minimum valid character (space)
    assert_eq!(is_valid(input), true);
}

#[test]
fn test_is_valid_above_tab() {
    let input: u8 = 33; // above tab and valid
    assert_eq!(is_valid(input), true);
}

#[test]
fn test_is_valid_exceeding_normal() {
    let input: u8 = 128; // above normal valid range
    assert_eq!(is_valid(input), true);
}

