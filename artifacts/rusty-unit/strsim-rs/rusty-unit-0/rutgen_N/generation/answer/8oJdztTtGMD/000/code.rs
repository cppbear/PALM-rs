// Answer 0

#[test]
fn test_jaro_winkler_equal_strings() {
    assert!((1.0 - jaro_winkler("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_different_strings() {
    assert!((0.0 - jaro_winkler("aaa", "bbb")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_similar_strings() {
    assert!((0.8 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_prefix_boost() {
    assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_string() {
    assert!((0.0 - jaro_winkler("", "nonempty")).abs() < 0.001);
    assert!((0.0 - jaro_winkler("nonempty", "")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_both_empty() {
    assert!((1.0 - jaro_winkler("", "")).abs() < 0.001);
}

