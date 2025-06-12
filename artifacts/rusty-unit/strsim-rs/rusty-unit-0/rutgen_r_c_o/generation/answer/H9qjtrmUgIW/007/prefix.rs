// Answer 0

#[test]
fn test_sorensen_dice_different_bigrams() {
    let a = "ab";
    let b = "cd";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_same_length_different_bigrams() {
    let a = "xy";
    let b = "zw";
    sorensen_dice(a, b);
}

#[test]
fn test_sorensen_dice_minimum_length_no_match() {
    let a = "pq";
    let b = "rs";
    sorensen_dice(a, b);
}

