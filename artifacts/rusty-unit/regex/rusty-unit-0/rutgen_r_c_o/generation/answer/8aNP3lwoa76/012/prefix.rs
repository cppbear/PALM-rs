// Answer 0

#[test]
fn test_class_query_binary() {
    let query = ClassQuery::Binary("ASCII");
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_assigned() {
    let query = ClassQuery::GeneralCategory("Assigned");
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_any() {
    let query = ClassQuery::GeneralCategory("Any");
    let _ = class(query);
}

#[test]
fn test_class_query_by_value_age() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let _ = class(query);
}

#[test]
fn test_class_query_by_value_script_extensions() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let _ = class(query);
}

