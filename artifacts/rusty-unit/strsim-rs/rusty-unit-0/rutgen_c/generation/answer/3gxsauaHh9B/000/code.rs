// Answer 0

#[test]
fn test_normalized_levenshtein_identical_strings() {
    assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_different_strings() {
    assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_strings() {
    assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_and_non_empty() {
    assert!(normalized_levenshtein("", "second").abs() < 0.00001);
    assert!(normalized_levenshtein("first", "").abs() < 0.00001);
}

