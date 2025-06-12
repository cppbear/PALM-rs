// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_empty_strings() {
    assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_first_empty() {
    assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_second_empty() {
    assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_same_strings() {
    assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_different_strings() {
    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
}

