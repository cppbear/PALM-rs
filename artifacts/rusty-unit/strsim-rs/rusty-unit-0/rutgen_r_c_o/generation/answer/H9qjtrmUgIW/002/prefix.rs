// Answer 0

#[test]
fn test_sorensen_dice_case_different_single_chars() {
    sorensen_dice("a", "b");
}

#[test]
fn test_sorensen_dice_case_empty_and_single_char() {
    sorensen_dice("", "a");
}

#[test]
fn test_sorensen_dice_case_single_char_and_empty() {
    sorensen_dice("a", "");
}

#[test]
fn test_sorensen_dice_case_single_char_and_another_single_char() {
    sorensen_dice("x", "y");
}

#[test]
fn test_sorensen_dice_case_two_distinct_single_chars() {
    sorensen_dice("z", "m");
}

