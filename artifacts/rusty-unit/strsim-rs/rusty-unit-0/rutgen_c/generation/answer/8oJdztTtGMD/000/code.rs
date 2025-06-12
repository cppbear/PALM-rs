// Answer 0

#[test]
fn test_jaro_winkler_identical_strings() {
    let result = jaro_winkler("test", "test");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_different_strings() {
    let result = jaro_winkler("abc", "xyz");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_similar_strings() {
    let result = jaro_winkler("cheeseburger", "cheese fries");
    assert!((0.866 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    let result = jaro_winkler("hello", "hallo");
    assert!((0.944 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_strings() {
    let result_a = jaro_winkler("", "");
    let result_b = jaro_winkler("a", "");
    assert!((1.0 - result_a).abs() < 0.001);
    assert!((0.0 - result_b).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_common_prefix() {
    let result = jaro_winkler("flaw", "lawn");
    assert!((0.82 - result).abs() < 0.001);
}

