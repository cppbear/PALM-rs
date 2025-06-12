// Answer 0

#[test]
fn test_canonicalize_general_category_assigned() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "Assigned",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_ascii() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "ASCII",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_any() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "any",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_general_category_custom() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "other",
    };
    let _ = query.canonicalize();
}

