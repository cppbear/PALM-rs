// Answer 0

#[test]
fn test_canonicalize_general_category() {
    struct ClassQuery {
        property_name: String,
        property_value: String,
    }

    impl ClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery, Error> {
            // Implementation would call the method under test
            unimplemented!()
        }
    }

    let class_query = ClassQuery {
        property_name: "General_Category".to_string(),
        property_value: "Lu".to_string(),
    };

    let result = class_query.canonicalize();
    assert!(result.is_ok());
    if let Ok(query) = result {
        match query {
            CanonicalClassQuery::GeneralCategory(canon) => {
                assert_eq!(canon, "Uppercase_Letter"); // Example expected value
            }
            _ => panic!("Expected GeneralCategory variant"),
        }
    }
}

#[test]
fn test_canonicalize_script() {
    struct ClassQuery {
        property_name: String,
        property_value: String,
    }

    impl ClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery, Error> {
            // Implementation would call the method under test
            unimplemented!()
        }
    }

    let class_query = ClassQuery {
        property_name: "Script".to_string(),
        property_value: "Latn".to_string(),
    };

    let result = class_query.canonicalize();
    assert!(result.is_ok());
    if let Ok(query) = result {
        match query {
            CanonicalClassQuery::Script(canon) => {
                assert_eq!(canon, "Latin"); // Example expected value
            }
            _ => panic!("Expected Script variant"),
        }
    }
}

#[test]
fn test_canonicalize_property_value_not_found() {
    struct ClassQuery {
        property_name: String,
        property_value: String,
    }

    impl ClassQuery {
        fn canonicalize(self) -> Result<CanonicalClassQuery, Error> {
            // Implementation would call the method under test
            unimplemented!()
        }
    }

    let class_query = ClassQuery {
        property_name: "General_Category".to_string(),
        property_value: "InvalidValue".to_string(),
    };

    let result = class_query.canonicalize();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error, Error::PropertyValueNotFound);
    }
}

