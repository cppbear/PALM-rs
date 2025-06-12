// Answer 0

#[test]
fn test_class_binary_property() {
    struct DummyPropertyValues;
    
    impl PropertyValues for DummyPropertyValues {
        fn by_name(name: &str) -> Option<&'static [(char, char)]> {
            match name {
                "Lowercase_Letter" => Some(&[('a', 'z')]),
                "Uppercase_Letter" => Some(&[('A', 'Z')]),
                _ => None,
            }
        }
    }

    let query_lowercase = ClassQuery::Binary("Lowercase_Letter");
    let query_uppercase = ClassQuery::Binary("Uppercase_Letter");

    let result_lowercase = class(query_lowercase);
    let result_uppercase = class(query_uppercase);

    assert!(result_lowercase.is_ok());
    assert!(result_uppercase.is_ok());
}

#[test]
fn test_class_script_property() {
    let query_latin = ClassQuery::Script("Latin");
    let query_cyrillic = ClassQuery::Script("Cyrillic");

    let result_latin = class(query_latin);
    let result_cyrillic = class(query_cyrillic);

    assert!(result_latin.is_ok());
    assert!(result_cyrillic.is_ok());
}

#[test]
fn test_class_general_category_property() {
    let query_ascii = ClassQuery::GeneralCategory("ASCII");
    let query_assigned = ClassQuery::GeneralCategory("Assigned");

    let result_ascii = class(query_ascii);
    let result_assigned = class(query_assigned);

    assert!(result_ascii.is_ok());
    assert!(result_assigned.is_ok());
}

#[should_panic]
#[test]
fn test_class_invalid_property() {
    let query_invalid = ClassQuery::Binary("Invalid_Property");
    let result_invalid = class(query_invalid);
    assert!(result_invalid.is_err());
}

