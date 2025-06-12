// Answer 0

#[test]
fn test_unicode_class_any() {
    let query = ClassQuery::GeneralCategory("Any");
    let result = class(query).unwrap();
    assert!(result.contains(&('\0', '\u{10FFFF}')));
}

#[test]
fn test_unicode_class_assigned() {
    let query = ClassQuery::GeneralCategory("Assigned");
    let result = class(query).unwrap();
    assert!(result.is_empty());
    // Assuming 'negate' implies it should not contain any unassigned characters
}

#[test]
fn test_unicode_class_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let result = class(query).unwrap();
    assert!(result.contains(&('\0', '\x7F')));
}

#[test]
fn test_unicode_class_script() {
    let query = ClassQuery::Script("Latin");
    let result = class(query).expect("Should return a valid class");
    // Assuming we can verify the characters for Latin script; this is a placeholder
    assert!(!result.is_empty());
}

#[test]
fn test_unicode_class_age() {
    let query = ClassQuery::ByValue { property_name: "Age", property_value: "5" };
    let result = class(query).expect("Should return a valid class");
    // Check that the result contains expected characters; this is a placeholder
    assert!(!result.is_empty());
}

#[test]
fn test_unicode_class_script_extensions() {
    let query = ClassQuery::ByValue { property_name: "Script_Extensions", property_value: "Latin" };
    let result = class(query).expect("Should return a valid class");
    // Assuming we can verify the characters for Latin script extensions; this is a placeholder
    assert!(!result.is_empty());
}

#[test]
#[should_panic(expected = "PropertyNotFound")]
fn test_unicode_class_invalid_canonical_class() {
    let query = ClassQuery::Binary("InvalidClassName");
    class(query).unwrap();
}

