// Answer 0

#[test]
fn test_canonicalize_script_property() {
    use ucd_util::PropertyValues;

    struct TestClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> TestClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            TestClassQuery { query }
        }
    }

    let query = TestClassQuery::new(ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Latin",
    });

    let result = query.query.canonicalize();
    
    match result {
        Ok(CanonicalClassQuery::Script(canon)) => {
            assert_eq!(canon, "Latin");  // Assuming "Latin" is a valid return from canonical_script
        }
        _ => panic!("Expected a Script CanonicalClassQuery"),
    }
}

#[test]
fn test_canonicalize_invalid_property_name() {
    struct TestClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> TestClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            TestClassQuery { query }
        }
    }

    let query = TestClassQuery::new(ClassQuery::ByValue {
        property_name: "InvalidPropertyName",
        property_value: "Latin",
    });

    let result = query.query.canonicalize();
    
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_canonicalize_invalid_property_value() {
    struct TestClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    impl<'a> TestClassQuery<'a> {
        fn new(query: ClassQuery<'a>) -> Self {
            TestClassQuery { query }
        }
    }

    let query = TestClassQuery::new(ClassQuery::ByValue {
        property_name: "Script",
        property_value: "InvalidValue",
    });

    let result = query.query.canonicalize();
    
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

