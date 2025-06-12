// Answer 0

#[test]
fn test_jaro_equal_strings() {
    let result = jaro("test", "test");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_different_strings() {
    let result = jaro("abc", "xyz");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_partial_similarity() {
    let result = jaro("hello", "hallo");
    assert!((0.8 - result).abs() < 0.001);
}

#[test]
fn test_jaro_empty_strings() {
    let result = jaro("", "");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_one_empty_string() {
    let result = jaro("not empty", "");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_long_strings() {
    let result = jaro("internationalization", "nationalization");
    assert!((0.822 - result).abs() < 0.001);
}

#[test]
fn test_jaro_different_cases() {
    let result = jaro("Hello", "hello");
    assert!((0.933 - result).abs() < 0.001);
}

