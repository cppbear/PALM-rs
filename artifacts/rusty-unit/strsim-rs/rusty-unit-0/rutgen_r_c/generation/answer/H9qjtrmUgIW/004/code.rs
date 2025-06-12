// Answer 0

#[test]
fn test_sorensen_dice_equal_strings() {
    assert_eq!(1.0, sorensen_dice("aa", "aa"));
}

#[test]
fn test_sorensen_dice_different_strings_with_bigrams() {
    assert_eq!(0.0, sorensen_dice("aa", "bb"));
}

#[test]
fn test_sorensen_dice_minimal_strings_with_no_common_bigrams() {
    assert_eq!(0.0, sorensen_dice("ab", "cd"));
}

#[test]
fn test_sorensen_dice_minimal_strings_with_common_bigrams() {
    assert_eq!(0.5, sorensen_dice("ab", "ab"));
}

#[test]
fn test_sorensen_dice_non_identical_bigrams_with_high_intersection() {
    assert_eq!(0.6666666666666666, sorensen_dice("ab", "ac"));
}

#[test]
fn test_sorensen_dice_non_identical_bigrams_with_no_intersection() {
    assert_eq!(0.0, sorensen_dice("az", "by"));
}

