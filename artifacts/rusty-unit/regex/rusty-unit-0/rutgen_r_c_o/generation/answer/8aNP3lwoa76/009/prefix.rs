// Answer 0

#[test]
fn test_class_query_binary_any() {
    let query = ClassQuery::Binary("Any");
    let _ = class(query);
}

#[test]
fn test_class_query_binary_assigned() {
    let query = ClassQuery::Binary("Assigned");
    let _ = class(query);
}

#[test]
fn test_class_query_binary_ascii() {
    let query = ClassQuery::Binary("ASCII");
    let _ = class(query);
}

#[test]
fn test_class_query_one_letter() {
    let query = ClassQuery::OneLetter('A');
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_any() {
    let query = ClassQuery::Binary("General_Category");
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_assigned() {
    let query = ClassQuery::Binary("General_Category");
    let _ = class(query);
}

#[test]
fn test_class_query_general_category_ascii() {
    let query = ClassQuery::Binary("General_Category");
    let _ = class(query);
}

