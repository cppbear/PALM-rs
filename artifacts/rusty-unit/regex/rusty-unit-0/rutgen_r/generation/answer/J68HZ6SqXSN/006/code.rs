// Answer 0

#[test]
fn test_canonicalize_property_value_not_found() {
    use regex_syntax::ClassQuery;
    use regex_syntax::{CanonicalClassQuery, Error};

    struct MockPropertyValues;
    impl MockPropertyValues {
        fn property_values(name: &str) -> Option<Vec<String>> {
            if name == "Supported_Property" {
                Some(vec!["Value1".to_string(), "Value2".to_string()])
            } else {
                None
            }
        }

        fn normalize(value: String) -> String {
            value
        }

        fn canonical_prop(property_name: &str) -> Option<&str> {
            if property_name == "Some_Property" {
                Some("Supported_Property")
            } else {
                None
            }
        }

        fn canonical_value(vals: Vec<String>, property_value: &str) -> Option<String> {
            None // To trigger the PropertyValueNotFound error
        }
    }

    // Test case where constraints are satisfied
    let property_name = "Some_Property".to_string();
    let property_value = "Value3".to_string(); // Not a valid value to match canonical_value

    let query = ClassQuery::ByValue {
        property_name: MockPropertyValues::normalize(property_name),
        property_value: MockPropertyValues::normalize(property_value),
    };

    let result = query.canonicalize();

    match result {
        Err(Error::PropertyValueNotFound) => assert!(true), // Test passes
        _ => panic!("Expected Err(Error::PropertyValueNotFound)"),
    }
}

