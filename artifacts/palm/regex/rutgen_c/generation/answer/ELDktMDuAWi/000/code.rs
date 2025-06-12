// Answer 0

#[test]
fn test_property_set_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("alpha", &[( 'a', 'a' )]),
        ("beta", &[( 'b', 'b' )]),
        ("gamma", &[( 'g', 'g' )]),
    ];
    
    let result = property_set(NAME_MAP, "beta");
    assert_eq!(result, Some(&[( 'b', 'b' )]));
}

#[test]
fn test_property_set_not_found() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("alpha", &[( 'a', 'a' )]),
        ("beta", &[( 'b', 'b' )]),
        ("gamma", &[( 'g', 'g' )]),
    ];
    
    let result = property_set(NAME_MAP, "delta");
    assert_eq!(result, None);
}

#[test]
fn test_property_set_empty() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[];
    
    let result = property_set(NAME_MAP, "alpha");
    assert_eq!(result, None);
}

#[test]
fn test_property_set_boundaries() {
    static NAME_MAP: &[(&str, &[(char, char)])] = &[
        ("a", &[( 'a', 'a' )]),
        ("z", &[( 'z', 'z' )]),
    ];
    
    let result_a = property_set(NAME_MAP, "a");
    let result_z = property_set(NAME_MAP, "z");
    
    assert_eq!(result_a, Some(&[( 'a', 'a' )]));
    assert_eq!(result_z, Some(&[( 'z', 'z' )]));

    let result_not_found = property_set(NAME_MAP, "b");
    assert_eq!(result_not_found, None);
}

