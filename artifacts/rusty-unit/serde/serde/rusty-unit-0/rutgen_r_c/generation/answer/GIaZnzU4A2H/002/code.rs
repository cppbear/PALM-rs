// Answer 0

#[test]
fn test_serialize_tuple_variant_success() {
    struct MockMap {
        key: Option<String>,
        value: Option<()>,
    }

    impl Serializer for FlatMapSerializer<'_, MockMap> {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<Self::Ok, Self::Error>;
        type SerializeTuple = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeMap = MockMap;
        type SerializeStruct = Impossible<Self::Ok, Self::Error>;
        type SerializeTupleVariant = FlatMapSerializeTupleVariantAsMapValue<'_, MockMap>;
        type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

        fn serialize_key(&mut self, key: &'static str) -> Result<Self::Ok, Self::Error> {
            self.key = Some(key.to_string());
            Ok(())
        }

        fn serialize_unit_variant(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.value = Some(());
            Ok(())
        }
    }

    let mut map = MockMap { key: None, value: None };
    let serializer = FlatMapSerializer(&mut map);
    let variant = "variant_name";

    let result = serializer.serialize_tuple_variant("example", 0, variant, 0);

    assert!(result.is_ok());
    let flat_map_variant_value = result.unwrap();
    assert_eq!(map.key, Some(variant.to_string()));
}

