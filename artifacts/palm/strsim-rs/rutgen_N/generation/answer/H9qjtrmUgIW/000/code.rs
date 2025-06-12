// Answer 0

#[test]
fn test_sorensen_dice_empty_strings() {
    assert_eq!(1.0, sorensen_dice("", ""));
}

#[test]
fn test_sorensen_dice_one_empty_string() {
    assert_eq!(0.0, sorensen_dice("", "a"));
    assert_eq!(0.0, sorensen_dice("a", ""));
}

#[test]
fn test_sorensen_dice_different_strings() {
    assert_eq!(0.0, sorensen_dice("french", "quebec"));
}

#[test]
fn test_sorensen_dice_identical_strings() {
    assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
}

#[test]
fn test_sorensen_dice_similar_strings() {
    assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
}

#[test]
fn test_sorensen_dice_two_characters() {
    assert_eq!(0.0, sorensen_dice("a", "b"));
    assert_eq!(1.0, sorensen_dice("aa", "aa"));
    assert_eq!(0.5, sorensen_dice("ab", "aa"));
}

