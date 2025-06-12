// Answer 0

#[test]
fn test_canonicalize_by_value_property_value_not_found() {
    struct MockClassQuery<'a> {
        query: ClassQuery<'a>,
    }

    let property_name = "Custom_Property";
    let property_value = "Custom_Value";

    let query = MockClassQuery {
        query: ClassQuery::ByValue {
            property_name,
            property_value,
        },
    };

    let result = query.query.canonicalize();
    
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

