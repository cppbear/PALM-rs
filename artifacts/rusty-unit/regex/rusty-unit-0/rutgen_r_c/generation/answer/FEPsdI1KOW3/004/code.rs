// Answer 0

#[test]
fn test_from_name_blank() {
    assert_eq!(ClassAsciiKind::from_name("blank"), Some(ClassAsciiKind::Blank));
}

#[test]
fn test_from_name_non_matching_cases() {
    assert_eq!(ClassAsciiKind::from_name("alnum"), None);
    assert_eq!(ClassAsciiKind::from_name("alpha"), None);
    assert_eq!(ClassAsciiKind::from_name("ascii"), None);
}

