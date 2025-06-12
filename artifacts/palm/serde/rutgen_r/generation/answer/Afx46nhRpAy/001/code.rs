// Answer 0

#[derive(Debug)]
struct UntaggedUnitVisitor<'a> {
    type_name: &'a str,
    variant_name: &'a str,
}

fn new<'a>(type_name: &'a str, variant_name: &'a str) -> UntaggedUnitVisitor<'a> {
    UntaggedUnitVisitor {
        type_name,
        variant_name,
    }
}

#[test]
fn test_new_valid_inputs() {
    let visitor = new("ExampleType", "ExampleVariant");
    assert_eq!(visitor.type_name, "ExampleType");
    assert_eq!(visitor.variant_name, "ExampleVariant");
}

#[test]
fn test_new_empty_type_name() {
    let visitor = new("", "NonEmptyVariant");
    assert_eq!(visitor.type_name, "");
    assert_eq!(visitor.variant_name, "NonEmptyVariant");
}

#[test]
fn test_new_empty_variant_name() {
    let visitor = new("NonEmptyType", "");
    assert_eq!(visitor.type_name, "NonEmptyType");
    assert_eq!(visitor.variant_name, "");
}

#[test]
fn test_new_both_empty() {
    let visitor = new("", "");
    assert_eq!(visitor.type_name, "");
    assert_eq!(visitor.variant_name, "");
}

#[test]
#[should_panic]
fn test_new_null_type_name() {
    new(std::ptr::null(), "NonEmptyVariant");
}

#[test]
#[should_panic]
fn test_new_null_variant_name() {
    new("NonEmptyType", std::ptr::null());
}

#[test]
#[should_panic]
fn test_new_null_both() {
    new(std::ptr::null(), std::ptr::null());
}

