// Answer 0

#[test]
fn test_normalized_levenshtein_equal_strings() {
    let result = normalized_levenshtein("hello", "hello");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_one_empty_string() {
    let result_empty_first = normalized_levenshtein("non-empty", "");
    assert!((result_empty_first - 0.0).abs() < 0.00001);
    
    let result_empty_second = normalized_levenshtein("", "non-empty");
    assert!((result_empty_second - 0.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_different_strings() {
    let result = normalized_levenshtein("kitten", "sitting");
    assert!((result - 0.57142).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_strings() {
    let result = normalized_levenshtein("", "");
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_partial_match() {
    let result = normalized_levenshtein("flaw", "lawn");
    assert!((result - 0.28571).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_large_difference() {
    let result = normalized_levenshtein("abcdef", "xyz");
    assert!((result - 0.0).abs() < 0.00001);
}

