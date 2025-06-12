// Answer 0

#[test]
fn test_from_name_none_for_invalid_names() {
    assert_eq!(ClassAsciiKind::from_name("not_a_kind"), None);
    assert_eq!(ClassAsciiKind::from_name("invalid"), None);
    assert_eq!(ClassAsciiKind::from_name(""), None);
    assert_eq!(ClassAsciiKind::from_name("random_string"), None);
    assert_eq!(ClassAsciiKind::from_name("!@#"), None);
    assert_eq!(ClassAsciiKind::from_name(" "), None);
    assert_eq!(ClassAsciiKind::from_name("123"), None);
    assert_eq!(ClassAsciiKind::from_name("alnum "), None);
    assert_eq!(ClassAsciiKind::from_name(" alnum"), None);
    assert_eq!(ClassAsciiKind::from_name("mixOfValidAndInvalid"), None);
    assert_eq!(ClassAsciiKind::from_name("ALPHABET"), None);
    assert_eq!(ClassAsciiKind::from_name("space1"), None);
    assert_eq!(ClassAsciiKind::from_name("upper_bound"), None);
    assert_eq!(ClassAsciiKind::from_name("digit1"), None);
}

#[test]
fn test_from_name_edge_cases() {
    assert_eq!(ClassAsciiKind::from_name("abc"), None);
    assert_eq!(ClassAsciiKind::from_name("zxy"), None);
    assert_eq!(ClassAsciiKind::from_name("  "), None);
    assert_eq!(ClassAsciiKind::from_name("\n"), None);
}

