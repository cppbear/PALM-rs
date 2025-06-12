// Answer 0

#[test]
fn test_normalized_levenshtein_equal_strings() {
    assert!((normalized_levenshtein("same", "same") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_different_strings() {
    assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_both() {
    let result = normalized_levenshtein("", "");
    assert_eq!(result, 1.0);
}

#[test]
fn test_normalized_levenshtein_first_empty() {
    let result = normalized_levenshtein("first", "");
    assert!(result < 0.00001);
}

#[test]
fn test_normalized_levenshtein_second_empty() {
    let result = normalized_levenshtein("", "second");
    assert!(result < 0.00001);
}

#[test]
fn test_normalized_levenshtein_same_length_different_strings() {
    assert!((normalized_levenshtein("abcde", "abfde") - 0.8).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_one_char_strings() {
    assert!((normalized_levenshtein("a", "b") - 0.0).abs() < 0.00001);
}

