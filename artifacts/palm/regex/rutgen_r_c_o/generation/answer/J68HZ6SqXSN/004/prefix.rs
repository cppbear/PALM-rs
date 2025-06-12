// Answer 0

#[test]
fn test_canonicalize_script_property_value_not_found() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "InvalidValue",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_script_property_value_not_found_with_spaces() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "  InvalidValue  ",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_script_property_value_not_found_numeric() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "12345",
    };
    let result = query.canonicalize();
}

#[test]
fn test_canonicalize_script_property_value_not_found_special_chars() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "!@#$%^&*()",
    };
    let result = query.canonicalize();
}

