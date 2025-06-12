// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    struct TestSerializer;
    
    impl TestSerializer {
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Content, ()> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_unit_variant("EnumName", 0, "VariantA");
    assert_eq!(result, Ok(Content::UnitVariant("EnumName", 0, "VariantA")));
}

#[test]
fn test_serialize_unit_variant_max_index() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Content, ()> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }
    }

    let serializer = TestSerializer;

    let max_index = u32::MAX;
    let result = serializer.serialize_unit_variant("EnumName", max_index, "VariantB");
    assert_eq!(result, Ok(Content::UnitVariant("EnumName", max_index, "VariantB")));
}

