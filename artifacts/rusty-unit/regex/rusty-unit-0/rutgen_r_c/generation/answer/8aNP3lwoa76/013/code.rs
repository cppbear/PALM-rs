// Answer 0

#[test]
fn test_class_binary_property() {
    let query = ClassQuery::Binary("Uppercase");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_assigned() {
    let query = ClassQuery::GeneralCategory("Assigned");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_any() {
    let query = ClassQuery::GeneralCategory("Any");
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_general_category_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_non_existent() {
    let query = ClassQuery::GeneralCategory("NonExistentCategory");
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_by_value_age() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_script_extensions() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_invalid_age() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "InvalidAge",
    };
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_property_not_found() {
    let query = ClassQuery::Binary("NonExistentProperty");
    let result = class(query);
    assert!(result.is_err());
}

