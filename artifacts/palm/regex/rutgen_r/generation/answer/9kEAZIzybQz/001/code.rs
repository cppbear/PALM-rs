// Answer 0

#[test]
fn test_canonical_script_valid() {
    let normalized_value = "Latin";
    let result = canonical_script(normalized_value);
    assert_eq!(result, Some("Latn"));
}

#[test]
fn test_canonical_script_invalid() {
    let normalized_value = "UnknownScript";
    let result = canonical_script(normalized_value);
    assert_eq!(result, None);
}

#[should_panic]
#[test]
fn test_canonical_script_panic() {
    // Simulate the conditions under which `property_values("Script").unwrap()` would panic.
    // This would typically mean invoking this function when the property values can't be loaded,
    // which is outside the test's control
    let _ = canonical_script("ScriptThatWillPanic");
}

