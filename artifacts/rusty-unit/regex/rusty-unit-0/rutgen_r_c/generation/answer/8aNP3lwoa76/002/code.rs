// Answer 0

#[test]
fn test_class_query_property_not_found() {
    use regex_syntax::class;
    use regex_syntax::ClassQuery;

    // Test non-existent binary property
    let query = ClassQuery::Binary("NonExistentProperty");
    let result = class(query);
    assert_eq!(result, Err(regex_syntax::Error::PropertyNotFound));

    // Test non-existent general category
    let query = ClassQuery::GeneralCategory("NonExistentCategory");
    let result = class(query);
    assert_eq!(result, Err(regex_syntax::Error::PropertyValueNotFound));

    // Test non-existent ByValue property
    let query = ClassQuery::ByValue {
        property_name: "NonExistentProperty",
        property_value: "NonExistentValue",
    };
    let result = class(query);
    assert_eq!(result, Err(regex_syntax::Error::PropertyNotFound));
}

