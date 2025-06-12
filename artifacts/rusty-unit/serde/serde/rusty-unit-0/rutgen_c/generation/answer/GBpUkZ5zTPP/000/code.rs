// Answer 0

#[test]
fn test_internally_tagged_unit_visitor_new() {
    let type_name = "TestType";
    let variant_name = "TestVariant";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_internally_tagged_unit_visitor_new_empty_names() {
    let type_name = "";
    let variant_name = "";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_internally_tagged_unit_visitor_new_special_chars() {
    let type_name = "!@#$%^&*()";
    let variant_name = "[]{}|";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

