// Answer 0

#[test]
fn test_class_binary() {
    struct TestQuery {
        // Define fields necessary for ClassQuery
    }

    impl ClassQuery for TestQuery {
        // Provide necessary method implementations
    }

    let query = TestQuery { /* initialize fields */ };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_age() {
    struct TestQuery {
        // Define fields necessary for ClassQuery
    }

    impl ClassQuery for TestQuery {
        // Provide necessary method implementations
    }

    let query = TestQuery { /* initialize with property_name: "Age", property_value */ };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_script_extensions() {
    struct TestQuery {
        // Define fields necessary for ClassQuery
    }

    impl ClassQuery for TestQuery {
        // Provide necessary method implementations
    }

    let query = TestQuery { /* initialize with property_name: "Script_Extensions", property_value */ };
    let result = class(query);
    assert!(result.is_ok());
}

