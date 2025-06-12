// Answer 0

#[test]
fn test_property_set_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("alnum", &[(char::from(0x30), char::from(0x39)), (char::from(0x41), char::from(0x5A)), (char::from(0x61), char::from(0x7A))]),
        ("digit", &[(char::from(0x30), char::from(0x39))]),
    ];
    let canonical = "alnum";
    let result = property_set(NAME_MAP, canonical);
    assert!(result.is_some());
    assert_eq!(result, Some(&[(char::from(0x30), char::from(0x39)), (char::from(0x41), char::from(0x5A)), (char::from(0x61), char::from(0x7A))]));
}

#[test]
fn test_property_set_not_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("alnum", &[(char::from(0x30), char::from(0x39)), (char::from(0x41), char::from(0x5A)), (char::from(0x61), char::from(0x7A))]),
        ("digit", &[(char::from(0x30), char::from(0x39))]),
    ];
    let canonical = "nonexistent";
    let result = property_set(NAME_MAP, canonical);
    assert!(result.is_none());
}

#[test]
fn test_property_set_empty_name_map() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[];
    let canonical = "alnum";
    let result = property_set(NAME_MAP, canonical);
    assert!(result.is_none());
}

