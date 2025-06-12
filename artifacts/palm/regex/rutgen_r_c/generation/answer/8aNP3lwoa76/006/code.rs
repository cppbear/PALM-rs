// Answer 0

#[test]
fn test_class_binary_property_found() {
    let query = ClassQuery::Binary("Lu");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_binary_property_not_found() {
    let query = ClassQuery::Binary("NonExistent");
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_class_by_value_age_property_found() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_by_value_age_property_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "NonExistent",
    };
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_class_by_value_script_extensions_property_found() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latn",
    };
    let result = class(query);
    assert!(result.is_ok());
}

