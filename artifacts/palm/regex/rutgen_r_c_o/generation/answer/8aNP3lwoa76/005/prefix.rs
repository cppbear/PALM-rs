// Answer 0

#[test]
fn test_class_query_binary() {
    let query = ClassQuery::Binary("SomeBinaryProperty");
    class(query);
}

#[test]
fn test_class_query_age_v1_1() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    class(query);
}

#[test]
fn test_class_query_age_v10_0() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V10_0",
    };
    class(query);
}

#[test]
fn test_class_query_script_extensions_example() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "ExampleScript",
    };
    class(query);
}

