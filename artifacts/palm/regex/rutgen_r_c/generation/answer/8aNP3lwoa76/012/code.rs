// Answer 0

#[test]
fn test_class_binary_property_found() {
    let query = ClassQuery::Binary("Lu");
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
fn test_class_general_category_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let expected = hir_class(&[('\0', '\x7F')]);
    let result = class(query).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_class_general_category_any_fail() {
    let query = ClassQuery::GeneralCategory("Any");
    let result = class(query);
    assert!(result.is_err());
}

#[test]
fn test_class_general_category_unassigned_fail() {
    let query = ClassQuery::GeneralCategory("Unassigned");
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
    assert!(result.is_ok()); // The specific result can vary, you could assert with specific values if you know expected ranges.
}

#[test]
fn test_class_by_value_script_extensions_fail() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Invalid_Value",
    };
    let result = class(query);
    assert!(result.is_err());
}

