// Answer 0

#[test]
fn test_canonicalize_by_value_valid() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn get(&self, _value: &str) -> Option<&'static str> {
            Some("CanonicalValue") // Mock return value
        }
    }

    let property_values = |_: &str| Some(DummyPropertyValues);

    let query = ClassQuery::ByValue {
        property_name: "SomeProperty",
        property_value: "SomeValue",
    };

    let result = query.canonicalize();
    assert_eq!(
        result,
        Ok(CanonicalClassQuery::ByValue {
            property_name: "CanonicalPropertyName",
            property_value: "CanonicalValue",
        })
    );
}

#[test]
#[should_panic]
fn test_canonicalize_by_value_property_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "NonExistentProperty",
        property_value: "SomeValue",
    };

    let _ = query.canonicalize(); // Should panic due to PropertyNotFound
}

#[test]
#[should_panic]
fn test_canonicalize_by_value_property_value_not_found() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn get(&self, _value: &str) -> Option<&'static str> {
            None // Mock return value leading to not found
        }
    }

    let query = ClassQuery::ByValue {
        property_name: "SomeProperty",
        property_value: "NonExistentValue",
    };

    let _ = query.canonicalize(); // Should panic due to PropertyValueNotFound
}

#[test]
fn test_canonicalize_by_value_fallback() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn get(&self, _value: &str) -> Option<&'static str> {
            Some("FallbackValue") // Mock return value
        }
    }

    let property_values = |_: &str| Some(DummyPropertyValues);

    let query = ClassQuery::ByValue {
        property_name: "ProvidedProperty",
        property_value: "FallbackValue",
    };

    let result = query.canonicalize();
    assert_eq!(
        result,
        Ok(CanonicalClassQuery::ByValue {
            property_name: "CanonicalPropertyName",
            property_value: "FallbackValue",
        })
    );
}

