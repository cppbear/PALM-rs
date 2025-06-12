// Answer 0

#[test]
fn test_class_binary_query() {
    let query = ClassQuery::Binary("A");
    let _ = class(query);
}

#[test]
fn test_class_by_value_age_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_age_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "Invalid_Age_Value",
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_script_extensions_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let _ = class(query);
}

#[test]
fn test_class_by_value_script_extensions_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Invalid_Script_Value",
    };
    let _ = class(query);
}

