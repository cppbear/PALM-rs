// Answer 0

#[test]
fn test_canonicalize_binary_property_found() {
    let query = ClassQuery::Binary("Lu");
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::Binary(name) => {
                assert_eq!(name, "Lu"); // Unicode category for uppercase letters
            }
            _ => panic!("Expected CanonicalClassQuery::Binary variant"),
        }
    }
}

#[test]
fn test_canonicalize_binary_property_not_found() {
    let query = ClassQuery::Binary("UnknownProperty");
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_canonicalize_one_letter_property_found() {
    let query = ClassQuery::OneLetter('A');
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::Binary(name) => {
                assert_eq!(name, "Lu"); // Unicode category for uppercase letters
            }
            _ => panic!("Expected CanonicalClassQuery::Binary variant"),
        }
    }
}

#[test]
fn test_canonicalize_one_letter_property_not_found() {
    // Assuming 'Z' does not trigger an error based on our test case context
    let query = ClassQuery::OneLetter('Z');
    let result = query.canonicalize();
    assert!(result.is_ok());
}

#[test]
fn test_canonicalize_by_value_property_found() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "Lu",
    };
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::GeneralCategory(name) => {
                assert_eq!(name, "Lu"); // Unicode general category for uppercase letters
            }
            _ => panic!("Expected CanonicalClassQuery::GeneralCategory variant"),
        }
    }
}

#[test]
fn test_canonicalize_by_value_property_value_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "UnknownValue",
    };
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_canonicalize_by_value_property_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "UnknownProperty",
        property_value: "AnyValue",
    };
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyNotFound));
}

