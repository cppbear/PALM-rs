// Answer 0

#[test]
fn test_sorensen_dice_two_character_strings_no_intersection() {
    assert_eq!(sorensen_dice("ab", "cd"), 0.0);
}

#[test]
fn test_sorensen_dice_two_character_strings_with_intersection() {
    assert_eq!(sorensen_dice("ab", "bc"), 0.0);
}

#[test]
fn test_sorensen_dice_two_character_strings_equal() {
    assert_eq!(sorensen_dice("ab", "ab"), 1.0);
} 

#[test]
fn test_sorensen_dice_two_character_strings_one_intersection() {
    assert_eq!(sorensen_dice("ab", "ac"), 0.3333333333333333);
} 

#[test]
fn test_sorensen_dice_two_character_strings_both_same_bigrams() {
    assert_eq!(sorensen_dice("aa", "aa"), 1.0);
} 

#[test]
fn test_sorensen_dice_edge_case_with_spaces() {
    assert_eq!(sorensen_dice(" a", " a"), 1.0);
} 

#[test]
fn test_sorensen_dice_bigrams_not_found() {
    assert_eq!(sorensen_dice("ab", "ef"), 0.0);
} 

#[test]
fn test_sorensen_dice_single_character_bigrams_output_zero() {
    assert_eq!(sorensen_dice("a", "b"), 0.0);
} 

#[test]
fn test_sorensen_dice_identical_bigrams_ratio() {
    assert_eq!(sorensen_dice("ab", "ba"), 1.0);
} 

#[test]
fn test_sorensen_dice_multiple_equal_bigrams() {
    assert_eq!(sorensen_dice("abab", "abab"), 1.0);
} 

