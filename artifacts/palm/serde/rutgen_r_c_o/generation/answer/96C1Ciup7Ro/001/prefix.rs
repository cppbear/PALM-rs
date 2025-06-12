// Answer 0

#[test]
fn test_serialize_valid_cases() {
    struct DummySerializer;

    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = ();

        fn serialize_unit_variant(
            &self,
            _enum_name: &'static str,
            _variant_index: u32,
            _variant_name: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = DummySerializer;

    let variant1 = AdjacentlyTaggedEnumVariant {
        enum_name: "test_enum",
        variant_index: 0,
        variant_name: "TestVariant",
    };
    let _ = variant1.serialize(serializer);

    let variant2 = AdjacentlyTaggedEnumVariant {
        enum_name: "test_enum",
        variant_index: 1,
        variant_name: "AnotherVariant",
    };
    let _ = variant2.serialize(serializer);

    let variant3 = AdjacentlyTaggedEnumVariant {
        enum_name: "edge_enum",
        variant_index: 0,
        variant_name: "BoundaryVariant",
    };
    let _ = variant3.serialize(serializer);

    let variant4 = AdjacentlyTaggedEnumVariant {
        enum_name: "extreme_enum",
        variant_index: u32::MAX,
        variant_name: "MaxVariant",
    };
    let _ = variant4.serialize(serializer);
}

