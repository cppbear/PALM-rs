// Answer 0

#[test]
fn test_canonicalize_script_latin() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Latin",
    };
    query.canonicalize();
}

#[test]
fn test_canonicalize_script_cyrillic() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Cyrillic",
    };
    query.canonicalize();
}

#[test]
fn test_canonicalize_script_result() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Greek",
    };
    query.canonicalize();
}

#[test]
fn test_canonicalize_script_arabic() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Arabic",
    };
    query.canonicalize();
}

