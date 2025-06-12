// Answer 0

#[test]
fn test_class_query_binary() {
    use hir::ClassUnicode;
    use unicode_tables::property_bool;

    let query = ClassQuery::Binary("Letter");
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);

    let query = ClassQuery::Binary("NonExistentProperty");
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyNotFound));
}

#[test]
fn test_class_query_general_category_any() {
    let query = ClassQuery::GeneralCategory("Any");
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_class_query_general_category_assigned() {
    let query = ClassQuery::GeneralCategory("Assigned");
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_class_query_general_category_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_class_query_general_category_unassigned() {
    let query = ClassQuery::GeneralCategory("Unassigned");
    
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_class_query_by_value_age() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_class_query_by_value_script_extensions() {
    let query = ClassQuery::ByValue {
        property_name: "Script_Extensions",
        property_value: "Latin",
    };
    
    let result = class(query);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_class_query_by_value_invalid_property_name() {
    let query = ClassQuery::ByValue {
        property_name: "InvalidName",
        property_value: "SomeValue",
    };
    
    let result = class(query);
    assert_eq!(result, Err(Error::PropertyNotFound));
}

