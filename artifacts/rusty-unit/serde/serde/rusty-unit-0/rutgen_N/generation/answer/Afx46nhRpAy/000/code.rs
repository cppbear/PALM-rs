// Answer 0

#[test]
fn test_new_creates_untagged_unit_visitor() {
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

    let visitor = new("TestType", "TestVariant");
    assert_eq!(visitor.type_name, "TestType");
    assert_eq!(visitor.variant_name, "TestVariant");
}

