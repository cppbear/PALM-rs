// Answer 0

#[test]
fn test_property_set_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("age", &[( 'A', 'B')]),
        ("category", &[( 'C', 'D')]),
    ];
    
    let result = property_set(NAME_MAP, "age");
    assert_eq!(result, Some(&[( 'A', 'B')]));
}

#[test]
fn test_property_set_not_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("age", &[( 'A', 'B')]),
        ("category", &[( 'C', 'D')]),
    ];

    let result = property_set(NAME_MAP, "non_existing_property");
    assert_eq!(result, None);
}

#[test]
fn test_property_set_empty_name_map() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[];

    let result = property_set(NAME_MAP, "age");
    assert_eq!(result, None);
}

#[test]
fn test_property_set_boundary_case() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("a", &[( 'A', 'B')]),
        ("b", &[( 'C', 'D')]),
    ];

    let result = property_set(NAME_MAP, "a");
    assert_eq!(result, Some(&[( 'A', 'B')]));

    let result = property_set(NAME_MAP, "b");
    assert_eq!(result, Some(&[( 'C', 'D')]));
}

