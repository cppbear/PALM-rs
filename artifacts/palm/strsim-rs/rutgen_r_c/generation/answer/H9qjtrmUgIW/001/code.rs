// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("apple", "apple"));
    assert_eq!(1.0, sorensen_dice("test", "test"));
    assert_eq!(1.0, sorensen_dice("aaa", "aaa"));
    assert_eq!(1.0, sorensen_dice(" ", " "));
    assert_eq!(1.0, sorensen_dice("hello world", "hello world"));
}

#[test]
fn test_sorensen_dice_inner_spaces() {
    assert_eq!(1.0, sorensen_dice(" hello ", "hello"));
    assert_eq!(1.0, sorensen_dice("hello", " hello"));
    assert_eq!(1.0, sorensen_dice(" a b c ", "a b c"));
    assert_eq!(1.0, sorensen_dice("  test  ", "test"));
}

#[test]
fn test_sorensen_dice_empty_strings() {
    assert_eq!(1.0, sorensen_dice("", ""));
}

