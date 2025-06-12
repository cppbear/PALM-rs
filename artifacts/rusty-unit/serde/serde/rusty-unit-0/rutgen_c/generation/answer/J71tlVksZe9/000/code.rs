// Answer 0

#[test]
fn test_serialize_unit_variant() {
    struct MockMap {
        entries: Vec<(String, ())>,
    }

    impl MockMap {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn serialize_entry(&mut self, key: &str, value: &()) -> Result<(), ()> {
            self.entries.push((key.to_string(), *value));
            Ok(())
        }
    }
    
    impl SerializeMap for MockMap {
        type Error = ();
        
        fn serialize_key(&mut self, key: &str) -> Result<(), Self::Error> {
            self.serialize_entry(key, &())
        }
    }

    let mut map = MockMap::new();
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_unit_variant("TestVariant", 0, "VariantA");
    
    assert!(result.is_ok());
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].0, "VariantA");
}

