// Answer 0

#[test]
fn test_flat_map_serialize_tuple_variant_as_map_value_new() {
    struct MockMap {
        entries: Vec<(Content, Content)>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = ();

        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            self.entries.push((Content::String("key".to_string()), Content::String("value".to_string())));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { entries: Vec::new() };
    let flat_map = FlatMapSerializeTupleVariantAsMapValue::new(&mut map);

    assert_eq!(map.entries.len(), 0);
    assert!(flat_map.fields.is_empty());
}

