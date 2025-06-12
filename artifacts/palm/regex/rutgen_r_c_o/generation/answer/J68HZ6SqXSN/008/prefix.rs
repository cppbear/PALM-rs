// Answer 0

#[test]
fn test_canonicalize_with_nonexistent_property_name() {
    let query = ClassQuery::ByValue {
        property_name: "NonExistentProperty",
        property_value: "SomeValue",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_with_nonexistent_property_value() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "NonExistentValue",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_with_random_nonexistent_property() {
    let query = ClassQuery::ByValue {
        property_name: "AnotherNonExistentProp",
        property_value: "AnotherValue",
    };
    let _ = query.canonicalize();
}

