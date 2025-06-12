// Answer 0

#[test]
fn test_sorensen_dice_a_equals_b_condition_false_a_len_2_b_len_1() {
    let a = "ab";
    let b = "a";
    assert_eq!(0.0, sorensen_dice(a, b));
}

#[test]
fn test_sorensen_dice_a_equals_b_condition_false_a_len_2_b_len_0() {
    let a = "ab";
    let b = "";
    assert_eq!(0.0, sorensen_dice(a, b));
}

#[test]
fn test_sorensen_dice_a_equals_b_condition_false_a_len_2_b_also_empty_space() {
    let a = "ab";
    let b = " ";
    assert_eq!(0.0, sorensen_dice(a, b));
}

