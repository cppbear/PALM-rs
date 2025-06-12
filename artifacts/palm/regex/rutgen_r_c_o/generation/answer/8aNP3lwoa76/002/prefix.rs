// Answer 0

#[test]
fn test_class_with_non_existent_property() {
    let query = ClassQuery::ByValue {
        property_name: "NonExistentProperty",
        property_value: "SomeValue",
    };
    class(query);
}

#[test]
fn test_class_with_non_existent_binary() {
    let query = ClassQuery::Binary("Z");
    class(query);
}

#[test]
fn test_class_with_non_existent_general_category() {
    let query = ClassQuery::GeneralCategory("NonExistentCategory");
    class(query);
}

#[test]
fn test_class_with_non_existent_script() {
    let query = ClassQuery::Script("NonExistentScript");
    class(query);
}

