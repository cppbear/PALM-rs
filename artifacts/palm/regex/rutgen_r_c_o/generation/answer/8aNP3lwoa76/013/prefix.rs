// Answer 0

#[test]
fn test_class_binary_a() {
    let query = ClassQuery::Binary("a");
    let _ = class(query);
}

#[test]
fn test_class_binary_e() {
    let query = ClassQuery::Binary("e");
    let _ = class(query);
}

#[test]
fn test_class_general_category_letter() {
    let query = ClassQuery::Binary("Letter");
    let _ = class(query);
}

#[test]
fn test_class_general_category_number() {
    let query = ClassQuery::Binary("Number");
    let _ = class(query);
}

#[test]
fn test_class_general_category_symbol() {
    let query = ClassQuery::Binary("Symbol");
    let _ = class(query);
}

#[test]
fn test_class_general_category_punctuation() {
    let query = ClassQuery::Binary("Punctuation");
    let _ = class(query);
}

#[test]
fn test_class_invalid_binary() {
    let query = ClassQuery::Binary("invalid");
    let _ = class(query);
}

#[test]
fn test_class_general_category_invalid() {
    let query = ClassQuery::Binary("InvalidCategory");
    let _ = class(query);
}

