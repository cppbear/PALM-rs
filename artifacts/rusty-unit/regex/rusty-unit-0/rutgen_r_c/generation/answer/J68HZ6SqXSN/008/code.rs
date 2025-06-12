// Answer 0

#[test]
fn test_canonicalize_by_value_property_not_found() {
    struct TestClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    let test_instance = TestClassQuery {
        query: ClassQuery::ByValue {
            property_name: "non_existent_property",
            property_value: "some_value",
        },
    };

    let result = test_instance.query.canonicalize();
    assert_eq!(result, Err(Error::PropertyNotFound));
}

