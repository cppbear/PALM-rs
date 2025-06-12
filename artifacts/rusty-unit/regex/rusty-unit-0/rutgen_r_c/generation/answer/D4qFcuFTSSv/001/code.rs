// Answer 0

#[test]
fn test_normalize_empty_string() {
    let result = normalize("");
    assert_eq!(result, "");
}

#[test]
fn test_normalize_single_word() {
    let result = normalize("example");
    assert_eq!(result, "example");
}

#[test]
fn test_normalize_with_special_characters() {
    let result = normalize("example!@#");
    assert_eq!(result, "example!@#");
}

#[test]
fn test_normalize_with_unicode() {
    let result = normalize("éxample");
    assert_eq!(result, "éxample");
}

#[test]
fn test_normalize_with_symbolic_name() {
    let result = normalize("CJK UNIFIED IDEOGRAPHS");
    assert_eq!(result, "CJK UNIFIED IDEOGRAPH");
}

#[should_panic]
fn test_normalize_panic_case() {
    // This test is meant to show potential panic, define input based on expected behaviour.
    let _ = normalize("invalid input that might panic");
}

