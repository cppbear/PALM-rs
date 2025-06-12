// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_empty_a_non_empty_b() {
    let result = normalized_damerau_levenshtein("", "flower");
    assert!(result.abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_empty_a_empty_b() {
    let result = normalized_damerau_levenshtein("", "");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_non_empty_a_empty_b() {
    let result = normalized_damerau_levenshtein("tree", "");
    assert!(result.abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_identical_strings() {
    let result = normalized_damerau_levenshtein("sunglasses", "sunglasses");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_different_strings() {
    let result = normalized_damerau_levenshtein("levenshtein", "löwenbräu");
    assert!((result - 0.27272).abs() < 0.00001);
}

