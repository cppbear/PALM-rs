// Answer 0

#[test]
fn test_serialize_adjacent_tagged_enum_variant() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_unit_variant(
            self,
            _enum_name: &'static str,
            _variant_index: u32,
            _variant_name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let variant = AdjacentlyTaggedEnumVariant {
        enum_name: "TestEnum",
        variant_index: 0,
        variant_name: "TestVariant",
    };

    let serializer = MockSerializer;
    let result = variant.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_adjacent_tagged_enum_variant_boundary() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();

        fn serialize_unit_variant(
            self,
            _enum_name: &'static str,
            _variant_index: u32,
            _variant_name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let variant = AdjacentlyTaggedEnumVariant {
        enum_name: "BoundaryEnum",
        variant_index: u32::MAX,
        variant_name: "BoundaryVariant",
    };

    let serializer = MockSerializer;
    let result = variant.serialize(serializer);
    assert!(result.is_ok());
}

