// Answer 0

#[test]
fn test_canonicalize_by_value_valid() {
    let property_name = "SomeCanonicalPropertyName";
    let property_value = "SomeValidPropertyValue";
    
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_valid_with_different_values() {
    let property_name = "AnotherCanonicalPropertyName";
    let property_value = "AnotherValidPropertyValue";
    
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_edge_case() {
    let property_name = "EdgeCaseCanonicalPropertyName";
    let property_value = "EdgeCaseValidPropertyValue";
    
    let query = ClassQuery::ByValue {
        property_name,
        property_value,
    };
    
    let _ = query.canonicalize();
}

