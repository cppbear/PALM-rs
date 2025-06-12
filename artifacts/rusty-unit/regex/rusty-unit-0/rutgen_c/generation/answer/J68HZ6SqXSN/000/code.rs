// Answer 0

#[test]
fn test_canonicalize_one_letter() {
    let query = ClassQuery::OneLetter('a');
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::Binary(c) => assert_eq!(c, "BinaryNameForA"), // Replace with expected binary name for 'a'
            _ => panic!("Expected a Binary variant"),
        }
    }
}

#[test]
fn test_canonicalize_binary() {
    let query = ClassQuery::Binary("script");
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::Script(c) => assert_eq!(c, "CanonicalScriptName"), // Replace with expected canonical script name for "script"
            _ => panic!("Expected a Script variant"),
        }
    }
}

#[test]
fn test_canonicalize_by_value_valid() {
    let query = ClassQuery::ByValue { 
        property_name: "General_Category", 
        property_value: "Letter" 
    };
    let result = query.canonicalize();
    assert!(result.is_ok());
    if let Ok(canonical) = result {
        match canonical {
            CanonicalClassQuery::GeneralCategory(c) => assert_eq!(c, "Letter"), // Replace with expected canonical general category name for "Letter"
            _ => panic!("Expected a GeneralCategory variant"),
        }
    }
}

#[test]
fn test_canonicalize_by_value_invalid_property_name() {
    let query = ClassQuery::ByValue { 
        property_name: "InvalidProperty", 
        property_value: "SomeValue" 
    };
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_canonicalize_by_value_invalid_property_value() {
    let query = ClassQuery::ByValue { 
        property_name: "General_Category", 
        property_value: "InvalidValue" 
    };
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

