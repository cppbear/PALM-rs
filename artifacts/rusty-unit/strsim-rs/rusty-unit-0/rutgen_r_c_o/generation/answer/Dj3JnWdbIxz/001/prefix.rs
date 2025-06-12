// Answer 0

#[test]
fn test_bigrams_empty_string() {
    let input = "";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_single_character() {
    let input = "a";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_two_characters() {
    let input = "ab";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_three_characters() {
    let input = "abc";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_four_characters_repeating() {
    let input = "aabb";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_four_characters_unique() {
    let input = "abcd";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_five_characters_unique() {
    let input = "abcde";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_six_characters_repeating() {
    let input = "aabbcc";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_seven_characters_unique() {
    let input = "abcdefg";
    let result = bigrams(input).collect::<Vec<_>>();
}

#[test]
fn test_bigrams_eight_characters_repeating_pattern() {
    let input = "abcdabcd";
    let result = bigrams(input).collect::<Vec<_>>();
}

