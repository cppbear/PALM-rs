// Answer 0

#[test]
fn test_jaro_winkler_basic_case() {
    let result = jaro_winkler("cheeseburger", "cheese fries");
    assert!((0.866 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_identical_strings() {
    let result = jaro_winkler("test", "test");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_completely_different_strings() {
    let result = jaro_winkler("apple", "orange");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    let result = jaro_winkler("hello", "hallo");
    assert!((0.826 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_with_common_prefix() {
    let result = jaro_winkler("flying", "flyer");
    assert!((0.774 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_strings() {
    let result = jaro_winkler("", "");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_one_empty_string() {
    let result = jaro_winkler("test", "");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_with_different_case() {
    let result = jaro_winkler("Test", "test");
    assert!((0.866 - result).abs() < 0.001);
}

