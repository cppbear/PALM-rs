// Answer 0

#[test]
fn test_canonicalize_general_category_property_value_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "unknown",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_any_property_value_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "unknown",
    };
    let result = query.canonicalize();
}

