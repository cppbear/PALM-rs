// Answer 0

#[test]
fn test_class_with_binary_property() {
    let query = ClassQuery::Binary("L");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_age_property_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_age_property_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "InvalidVersion",
    };
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_class_with_script_extensions_property_valid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latn",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_script_extensions_property_invalid() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "InvalidScript",
    };
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

