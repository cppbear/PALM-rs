// Answer 0

#[test]
fn test_tuple_variant_zero_length_with_valid_visitor() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        forward_to_deserialize_any!();
    }

    let variant_access = UnitOnly;
    let visitor = TestVisitor;
    let len = 0;

    let _ = variant_access.tuple_variant(len, visitor);
}

#[test]
fn test_tuple_variant_zero_length_with_another_valid_visitor() {
    struct AnotherTestVisitor;

    impl<'de> de::Visitor<'de> for AnotherTestVisitor {
        type Value = ();

        forward_to_deserialize_any!();
    }

    let variant_access = UnitOnly;
    let visitor = AnotherTestVisitor;
    let len = 0;

    let _ = variant_access.tuple_variant(len, visitor);
}

