// Answer 0

#[test]
fn test_jaro_winkler_equal_strings() {
    let result = jaro_winkler("identical", "identical");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_completely_different_strings() {
    let result = jaro_winkler("abc", "xyz");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    let result = jaro_winkler("martha", "marhta");
    assert!((0.961 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_common_prefix_boost() {
    let result = jaro_winkler("cheeseburger", "cheese fries");
    assert!((0.866 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_strings() {
    let result = jaro_winkler("", "");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_one_empty_string() {
    let result_a = jaro_winkler("nonempty", "");
    assert!((0.0 - result_a).abs() < 0.001);
    let result_b = jaro_winkler("", "nonempty");
    assert!((0.0 - result_b).abs() < 0.001);
}

