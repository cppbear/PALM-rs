// Answer 0

#[test]
fn test_sorensen_dice_edge_case_non_empty_different_bigrams() {
    let a = "ab"; // a.len() == 2
    let b = "cd"; // b.len() == 2
    assert_eq!(0.0, sorensen_dice(a, b)); // no common bigrams
}

#[test]
fn test_sorensen_dice_edge_case_empty_and_non_empty() {
    let a = "ab"; // a.len() == 2
    let b = "";   // b.len() < 2, should return 0.0
    assert_eq!(0.0, sorensen_dice(a, b)); 
}

#[test]
fn test_sorensen_dice_edge_case_same_length_different() {
    let a = "xy"; // a.len() == 2
    let b = "zw"; // b.len() == 2
    assert_eq!(0.0, sorensen_dice(a, b)); // no common bigrams
}

#[test]
fn test_sorensen_dice_edge_case_identical() {
    let a = "aa"; // a.len() == 2
    let b = "aa"; // a == b, should return 1.0
    assert_eq!(1.0, sorensen_dice(a, b));
}

#[test]
fn test_sorensen_dice_similar_non_identical_bigrams() {
    let a = "ab"; // a.len() == 2
    let b = "ba"; // b.len() == 2
    assert_eq!(1.0, sorensen_dice(a, b)); // 1 common bigram
}

