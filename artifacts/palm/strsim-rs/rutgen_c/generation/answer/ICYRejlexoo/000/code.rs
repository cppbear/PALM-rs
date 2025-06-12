// Answer 0

#[test]
fn test_generic_jaro_winkler_identical_strings() {
    let a = "test".chars();
    let b = "test".chars();
    assert_eq!(generic_jaro_winkler(&a, &b), 1.0);
}

#[test]
fn test_generic_jaro_winkler_different_strings() {
    let a = "test".chars();
    let b = "best".chars();
    assert!(generic_jaro_winkler(&a, &b) < 1.0);
}

#[test]
fn test_generic_jaro_winkler_prefix_boost() {
    let a = "test".chars();
    let b = "teap".chars();
    let result = generic_jaro_winkler(&a, &b);
    assert!(result > generic_jaro(&a, &b));
}

#[test]
fn test_generic_jaro_winkler_empty_string() {
    let a = "".chars();
    let b = "".chars();
    assert_eq!(generic_jaro_winkler(&a, &b), 1.0);
}

#[test]
fn test_generic_jaro_winkler_one_empty_string() {
    let a = "test".chars();
    let b = "".chars();
    assert_eq!(generic_jaro_winkler(&a, &b), 0.0);
}

#[test]
fn test_generic_jaro_winkler_similar_strings() {
    let a = "test".chars();
    let b = "tost".chars();
    let result = generic_jaro_winkler(&a, &b);
    assert!(result > 0.0);
    assert!(result < 1.0);
}

#[test]
fn test_generic_jaro_winkler_case_sensitivity() {
    let a = "Test".chars();
    let b = "test".chars();
    assert!(generic_jaro_winkler(&a, &b) < 1.0);
}

