// Answer 0

#[derive(Debug)]
struct InternallyTaggedUnitVisitor<'a> {
    type_name: &'a str,
    variant_name: &'a str,
}

#[test]
fn test_new_function() {
    let type_name = "TypeA";
    let variant_name = "VariantA";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_new_function_with_empty_strings() {
    let type_name = "";
    let variant_name = "";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

#[test]
fn test_new_function_with_long_strings() {
    let type_name = "A really long type name that could be used.";
    let variant_name = "A really long variant name that could be used.";
    let visitor = InternallyTaggedUnitVisitor::new(type_name, variant_name);
    
    assert_eq!(visitor.type_name, type_name);
    assert_eq!(visitor.variant_name, variant_name);
}

