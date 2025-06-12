// Answer 0

#[test]
fn test_jaro_identical_strings() {
    let result = jaro("test", "test");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_empty_strings() {
    let result = jaro("", "");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_one_empty_string() {
    let result = jaro("test", "");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_different_strings() {
    let result = jaro("frog", "fog");
    assert!((0.833 - result).abs() < 0.001);
}

#[test]
fn test_jaro_long_similar_strings() {
    let result = jaro("night", "nacht");
    assert!((0.4 - result).abs() < 0.001);
}

#[test]
fn test_jaro_reverse_strings() {
    let result = jaro("abc", "cba");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_case_sensitive() {
    let result = jaro("Test", "test");
    assert!((0.0 - result).abs() < 0.001);
}

