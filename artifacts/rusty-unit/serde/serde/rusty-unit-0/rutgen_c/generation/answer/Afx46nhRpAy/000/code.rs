// Answer 0

#[test]
fn test_untagged_unit_visitor_new() {
    let type_name = "TestType";
    let variant_name = "TestVariant";
    
    let visitor = UntaggedUnitVisitor::new(type_name, variant_name);
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_untagged_unit_visitor_new_empty_names() {
    let type_name = "";
    let variant_name = "";
    
    let visitor = UntaggedUnitVisitor::new(type_name, variant_name);
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_untagged_unit_visitor_new_special_characters() {
    let type_name = "!@#$%^&*()_+";
    let variant_name = "`~|}{\"':;?><";
    
    let visitor = UntaggedUnitVisitor::new(type_name, variant_name);
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

