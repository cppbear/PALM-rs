// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("test", "test"));
    assert_eq!(1.0, sorensen_dice("abcd", "abcd"));
    assert_eq!(1.0, sorensen_dice("Sørensen", "Sørensen"));
    assert_eq!(1.0, sorensen_dice("  leading and trailing spaces  ", "  leading and trailing spaces  "));
    assert_eq!(1.0, sorensen_dice("12345", "12345"));
}

