// Answer 0

#[test]
fn test_sorensen_dice_empty_vs_single_char() {
    assert_eq!(0.0, sorensen_dice("", "a"));
}

#[test]
fn test_sorensen_dice_single_char_vs_empty() {
    assert_eq!(0.0, sorensen_dice("a", ""));
}

#[test]
fn test_sorensen_dice_single_char_vs_single_char() {
    assert_eq!(0.0, sorensen_dice("a", "b"));
}

#[test]
fn test_sorensen_dice_single_char_vs_two_chars() {
    assert_eq!(0.0, sorensen_dice("a", "ab"));
}

#[test]
fn test_sorensen_dice_long_string_vs_single_char() {
    assert_eq!(0.0, sorensen_dice("hello", "h"));
}

