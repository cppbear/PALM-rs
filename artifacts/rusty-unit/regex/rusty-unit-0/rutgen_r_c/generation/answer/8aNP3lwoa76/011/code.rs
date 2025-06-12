// Answer 0

#[test]
fn test_class_with_binary_property() {
    let query = ClassQuery::Binary("Alphabetic");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_assigned_general_category() {
    let query = ClassQuery::GeneralCategory("Assigned");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_ascii_general_category() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_any_general_category() {
    let query = ClassQuery::GeneralCategory("Any");
    let result = class(query);
    assert!(result.is_ok());
}

#[test]
fn test_class_with_unassigned_general_category() {
    let query = ClassQuery::GeneralCategory("Unassigned");
    let result = class(query);
    assert!(result.is_err());
}

