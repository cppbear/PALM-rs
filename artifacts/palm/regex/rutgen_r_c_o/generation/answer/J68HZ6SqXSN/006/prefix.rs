// Answer 0

#[test]
fn test_canonicalize_by_value_invalid_property_value() {
    let property_name = "ValidProperty1";
    let property_value = "InvalidValue1";
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_invalid_property_value_2() {
    let property_name = "ValidProperty2";
    let property_value = "InvalidValue2";
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    let _ = query.canonicalize();
}

