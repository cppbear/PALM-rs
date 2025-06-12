// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_same_strings() {
    let result = normalized_damerau_levenshtein("sunglasses", "sunglasses");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_empty_strings() {
    let result = normalized_damerau_levenshtein("", "");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_empty_first_string() {
    let result = normalized_damerau_levenshtein("", "flower");
    assert!(result < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_empty_second_string() {
    let result = normalized_damerau_levenshtein("tree", "");
    assert!(result < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_different_strings() {
    let result = normalized_damerau_levenshtein("levenshtein", "löwenbräu");
    assert!((result - 0.27272).abs() < 0.00001);
}

