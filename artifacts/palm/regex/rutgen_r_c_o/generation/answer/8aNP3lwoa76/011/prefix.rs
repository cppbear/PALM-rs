// Answer 0

#[test]
fn test_class_binary_property() {
    let query = ClassQuery::Binary("SomeBinaryProperty");
    let _ = class(query);
}

#[test]
fn test_class_general_category_assigned() {
    let query = ClassQuery::Binary("Assigned");
    let _ = class(query);
}

#[test]
fn test_class_general_category_unassigned() {
    let query = ClassQuery::Binary("Unassigned");
    let _ = class(query);
}

#[test]
fn test_class_general_category_ascii() {
    let query = ClassQuery::GeneralCategory("ASCII");
    let _ = class(query);
}

#[test]
fn test_class_general_category_any() {
    let query = ClassQuery::GeneralCategory("Any");
    let _ = class(query);
}

#[test]
fn test_class_general_category_assigned_special() {
    let query = ClassQuery::GeneralCategory("Assigned");
    let _ = class(query);
}

