// Answer 0

#[test]
fn test_sorensen_dice_empty_strings() {
    assert_eq!(0.0, sorensen_dice("", "a"));
}

#[test]
fn test_sorensen_dice_one_character_string() {
    assert_eq!(0.0, sorensen_dice("a", ""));
}

#[test]
fn test_sorensen_dice_not_equal_single_characters() {
    assert_eq!(0.0, sorensen_dice("a", "b"));
}

#[test]
fn test_sorensen_dice_single_character_vs_string() {
    assert_eq!(0.0, sorensen_dice("x", "xyz"));
}

#[test]
fn test_sorensen_dice_two_character_vs_different_two_characters() {
    assert_eq!(0.0, sorensen_dice("ab", "cd"));
}

