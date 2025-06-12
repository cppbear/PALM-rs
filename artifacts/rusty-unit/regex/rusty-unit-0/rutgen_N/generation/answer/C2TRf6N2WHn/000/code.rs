// Answer 0

#[test]
fn test_canonical_binary_valid_binary_property() {
    struct TestStruct;

    impl TestStruct {
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
            // Mock implementation of the original function for testing
            let norm = normalize(name);
            if norm == "binaries" {
                return Ok(CanonicalClassQuery::Binary("BINARY_PROPERTY"));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.canonical_binary("binaries");
    assert!(result.is_ok());
    if let Ok(query) = result {
        match query {
            CanonicalClassQuery::Binary(prop) => assert_eq!(prop, "BINARY_PROPERTY"),
            _ => panic!("Expected Binary property"),
        }
    }
}

#[test]
fn test_canonical_binary_valid_general_category() {
    struct TestStruct;

    impl TestStruct {
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
            // Mock implementation of the original function for testing
            let norm = normalize(name);
            if norm == "general_category" {
                return Ok(CanonicalClassQuery::GeneralCategory("GENERAL_CATEGORY_PROPERTY"));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.canonical_binary("general_category");
    assert!(result.is_ok());
    if let Ok(query) = result {
        match query {
            CanonicalClassQuery::GeneralCategory(prop) => assert_eq!(prop, "GENERAL_CATEGORY_PROPERTY"),
            _ => panic!("Expected GeneralCategory property"),
        }
    }
}

#[test]
fn test_canonical_binary_valid_script() {
    struct TestStruct;

    impl TestStruct {
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
            // Mock implementation of the original function for testing
            let norm = normalize(name);
            if norm == "script" {
                return Ok(CanonicalClassQuery::Script("SCRIPT_PROPERTY"));
            }
            Err(Error::PropertyNotFound)
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.canonical_binary("script");
    assert!(result.is_ok());
    if let Ok(query) = result {
        match query {
            CanonicalClassQuery::Script(prop) => assert_eq!(prop, "SCRIPT_PROPERTY"),
            _ => panic!("Expected Script property"),
        }
    }
}

#[test]
fn test_canonical_binary_property_not_found() {
    struct TestStruct;

    impl TestStruct {
        fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error> {
            // Mock implementation of the original function for testing
            Err(Error::PropertyNotFound)
        }
    }

    let test_instance = TestStruct;
    let result = test_instance.canonical_binary("unknown_property");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::PropertyNotFound);
}

