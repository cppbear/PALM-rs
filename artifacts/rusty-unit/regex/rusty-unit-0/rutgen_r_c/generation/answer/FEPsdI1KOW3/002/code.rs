// Answer 0

#[test]
fn test_class_ascii_kind_from_name() {
    // Test for the case where name is "alpha"
    let name = "alpha";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, Some(ClassAsciiKind::Alpha));
}

#[test]
fn test_class_ascii_kind_from_name_not_matching() {
    // Test for the case where name is not matching any variant, e.g., "alnum"
    let name = "alnum";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

