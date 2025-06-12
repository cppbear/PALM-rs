// Answer 0

#[test]
fn test_serialize_struct_variant() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<SerializeStructVariant, &'static str> {
            Ok(SerializeStructVariant {
                name: String::from(variant),
                map: Map::new(),
            })
        }
    }

    struct SerializeStructVariant {
        name: String,
        map: Map,
    }

    struct Map;

    impl Map {
        fn new() -> Self {
            Map
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_struct_variant("TestStruct", 0, "VariantA", 0);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, "VariantA");
}

