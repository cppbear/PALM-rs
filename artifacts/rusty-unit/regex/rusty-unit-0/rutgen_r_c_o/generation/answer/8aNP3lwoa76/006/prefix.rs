// Answer 0

#[test]
fn test_class_query_binary_any() {
    let query = ClassQuery::Binary("Any");
    class(query);
}

#[test]
fn test_class_query_by_value_age_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V10_0",
    };
    class(query);
}

#[test]
fn test_class_query_by_value_script_extensions_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "v10_0",
    };
    class(query);
}

#[test]
#[should_panic]
fn test_class_query_by_value_age_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "Invalid_Age",
    };
    class(query);
}

#[test]
fn test_class_query_by_value_script_extensions_multiple() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "V9_0",
    };
    class(query);
}

