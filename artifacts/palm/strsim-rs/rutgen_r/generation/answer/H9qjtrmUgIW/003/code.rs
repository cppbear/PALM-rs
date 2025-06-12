// Answer 0

#[test]
fn test_sorensen_dice_a_len_2_b_len_1() {
    assert_eq!(0.0, sorensen_dice("ab", "a"));
}

#[test]
fn test_sorensen_dice_a_len_2_b_len_0() {
    assert_eq!(0.0, sorensen_dice("ab", ""));
}

#[test]
fn test_sorensen_dice_a_len_2_b_len_3() {
    assert_eq!(0.0, sorensen_dice("ab", "abc"));
}

