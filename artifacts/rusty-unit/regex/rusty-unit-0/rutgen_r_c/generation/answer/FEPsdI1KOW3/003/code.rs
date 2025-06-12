// Answer 0

#[test]
fn test_from_name_ascii() {
    let name = "ascii";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, Some(ClassAsciiKind::Ascii));
}

#[test]
fn test_from_name_not_a_valid_name() {
    let name = "invalid_name";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

#[test]
fn test_from_name_empty_string() {
    let name = "";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

#[test]
fn test_from_name_uppercase() {
    let name = "ASCII";
    let result = ClassAsciiKind::from_name(name);
    assert_eq!(result, None);
}

