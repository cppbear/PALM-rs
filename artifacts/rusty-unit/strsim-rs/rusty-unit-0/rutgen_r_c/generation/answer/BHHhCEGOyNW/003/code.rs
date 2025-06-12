// Answer 0

#[test]
fn test_normalized_damerau_levenshtein_same_strings() {
    let result = normalized_damerau_levenshtein("teststring", "teststring");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_different_strings() {
    let result = normalized_damerau_levenshtein("kitten", "sitting");
    assert!((result - 0.0).abs() < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_partial_match() {
    let result = normalized_damerau_levenshtein("flaw", "lawn");
    assert!((result - 0.28571).abs() < 0.00001); // Expecting a normalized score close to 0.28571
}

#[test]
fn test_normalized_damerau_levenshtein_panic_condition() {
    let result = normalized_damerau_levenshtein("hello", "hell");
    assert!((result - 0.75).abs() < 0.00001); // 0.75 as the normalized distance
}

#[test]
fn test_normalized_damerau_levenshtein_empty_vs_non_empty() {
    let result = normalized_damerau_levenshtein("nonempty", "");
    assert!(result < 0.00001);
}

#[test]
fn test_normalized_damerau_levenshtein_long_strings() {
    let result = normalized_damerau_levenshtein("longerstringexample", "longeststringexample");
    assert!((result - 0.75).abs() < 0.00001); // Change according to calculation
}

