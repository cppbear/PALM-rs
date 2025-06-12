// Answer 0

#[test]
fn test_property_set_existing() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("BasicLatin", &[('A', 'Z'), ('a', 'z')]),
        ("Latin1Supplement", &[('À', 'Ö'), ('Ø', 'ÿ')]),
    ];
    
    let result = property_set(NAME_MAP, "BasicLatin");
    assert_eq!(result, Some(&[('A', 'Z'), ('a', 'z')][..]));
}

#[test]
fn test_property_set_non_existing() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("BasicLatin", &[('A', 'Z'), ('a', 'z')]),
        ("Latin1Supplement", &[('À', 'Ö'), ('Ø', 'ÿ')]),
    ];
    
    let result = property_set(NAME_MAP, "NonExisting");
    assert_eq!(result, None);
}

#[test]
fn test_property_set_first_element() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("BasicLatin", &[('A', 'Z'), ('a', 'z')]),
        ("Latin1Supplement", &[('À', 'Ö'), ('Ø', 'ÿ')]),
    ];
    
    let result = property_set(NAME_MAP, "BasicLatin");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &[('A', 'Z'), ('a', 'z')][..]);
}

#[test]
fn test_property_set_last_element() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("BasicLatin", &[('A', 'Z'), ('a', 'z')]),
        ("Latin1Supplement", &[('À', 'Ö'), ('Ø', 'ÿ')]),
    ];

    let result = property_set(NAME_MAP, "Latin1Supplement");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), &[('À', 'Ö'), ('Ø', 'ÿ')][..]);
}

#[test]
fn test_property_set_empty_name_map() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[];

    let result = property_set(NAME_MAP, "AnyName");
    assert_eq!(result, None);
}

