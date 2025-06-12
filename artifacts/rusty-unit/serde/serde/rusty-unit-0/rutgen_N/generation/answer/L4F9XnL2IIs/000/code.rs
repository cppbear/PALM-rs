// Answer 0

#[test]
fn test_serialize_unit_variant() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Content, &'static str> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }
    }

    enum Content {
        UnitVariant(&'static str, u32, &'static str),
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_unit_variant("EnumName", 0, "VariantName").unwrap();
    
    if let Content::UnitVariant(name, index, variant) = result {
        assert_eq!(name, "EnumName");
        assert_eq!(index, 0);
        assert_eq!(variant, "VariantName");
    } else {
        panic!("Expected a UnitVariant");
    }
}

#[test]
fn test_serialize_unit_variant_boundary() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Content, &'static str> {
            Ok(Content::UnitVariant(name, variant_index, variant))
        }
    }

    enum Content {
        UnitVariant(&'static str, u32, &'static str),
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_unit_variant("BoundaryEnum", u32::MAX, "BoundaryVariant").unwrap();
    
    if let Content::UnitVariant(name, index, variant) = result {
        assert_eq!(name, "BoundaryEnum");
        assert_eq!(index, u32::MAX);
        assert_eq!(variant, "BoundaryVariant");
    } else {
        panic!("Expected a UnitVariant");
    }
}

