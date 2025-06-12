// Answer 0

#[test]
fn test_canonicalize_property_value_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "Any",
        property_value: "assigned",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_property_value_not_found_another_case() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "non_existent_value",
    };
    let _ = query.canonicalize();
}

