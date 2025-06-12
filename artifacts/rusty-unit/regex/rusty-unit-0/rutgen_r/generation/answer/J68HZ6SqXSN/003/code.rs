// Answer 0

#[test]
fn test_canonicalize_script() {
    struct MockClassQuery;

    impl MockClassQuery {
        fn canonical_binary(&self, _: &str) -> Result<CanonicalClassQuery> {
            Ok(CanonicalClassQuery::Script("Latin".to_string()))
        }
    }

    let property_name = "Script".to_string();
    let property_value = "Latn".to_string(); // Assuming "Latn" can be normalized and found

    let class_query = ClassQuery::ByValue { 
        property_name: property_name.clone(), 
        property_value: property_value.clone() 
    };

    let result = class_query.canonicalize();

    assert!(result.is_ok());
    if let Ok(canonical_query) = result {
        match canonical_query {
            CanonicalClassQuery::Script(canon) => {
                assert_eq!(canon, "Latin".to_string());
            }
            _ => panic!("Expected CanonicalClassQuery::Script"),
        }
    }
}

#[test]
fn test_canonicalize_with_unsupported_generic() {
    struct MockClassQuery;

    impl MockClassQuery {
        fn canonical_binary(&self, _: &str) -> Result<CanonicalClassQuery> {
            Err(Error::PropertyValueNotFound)
        }
    }

    let property_name = "UnknownProperty".to_string(); // An unsupported property
    let property_value = "SomeValue".to_string();

    let class_query = ClassQuery::ByValue { 
        property_name: property_name.clone(), 
        property_value: property_value.clone() 
    };

    let result = class_query.canonicalize();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::PropertyValueNotFound);
}

