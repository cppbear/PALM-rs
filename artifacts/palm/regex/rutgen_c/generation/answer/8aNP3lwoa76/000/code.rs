// Answer 0

#[test]
fn test_class_one_letter() {
    let query = ClassQuery::OneLetter('E');
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_any() {
    let query = ClassQuery::Binary("Any");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_assigned_category() {
    let query = ClassQuery::Binary("Assigned");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_general_category_ascii() {
    let query = ClassQuery::Binary("ASCII");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_invalid_property() {
    let query = ClassQuery::Binary("Invalid");
    let result = class(query);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::PropertyNotFound);
}

#[test]
fn test_class_age_property() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "V1_1",
    };
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_invalid_age_property() {
    let query = ClassQuery::ByValue {
        property_name: "Age",
        property_value: "Invalid_Age",
    };
    let result = class(query);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::PropertyValueNotFound);
}

