// Answer 0

#[test]
fn test_canonicalize_general_category() {
    struct LocalClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> LocalClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            LocalClassQuery { query }
        }
    }

    let property_name = "General_Category";
    let property_value = "assigned"; // Expecting this to map to "Assigned"

    // Assume the helper functions return the expected values for this test
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };

    let result = query.canonicalize();
    assert!(result.is_ok());

    if let Ok(canonical_query) = result {
        match canonical_query {
            CanonicalClassQuery::GeneralCategory(canon) => {
                assert_eq!(canon, "Assigned");
            }
            _ => panic!("Expected a GeneralCategory variant"),
        }
    }
}

#[test]
fn test_canonicalize_invalid_property_not_found() {
    struct LocalClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> LocalClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            LocalClassQuery { query }
        }
    }
 
    let property_name = "Non_Existent_Property";
    let property_value = "assigned"; // Assume we are passing a valid value for this property

    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };

    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_canonicalize_general_category_value_not_found() {
    struct LocalClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> LocalClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            LocalClassQuery { query }
        }
    }

    let property_name = "General_Category";
    let property_value = "non_existing_value"; // Invalid value to force PropertyValueNotFound

    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };

    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

