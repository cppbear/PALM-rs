// Answer 0

#[test]
fn test_end_method() {
    struct MockMap {
        entries: Vec<(&'static str, &'static str)>,
    }
    
    impl SerializeMap for MockMap {
        type Error = ();
        
        fn serialize_entry(&mut self, key: &'static str, value: &impl Serialize) -> Result<(), Self::Error> {
            self.entries.push((key, value.serialize(&mut Serializer).unwrap()));
            Ok(())
        }
        
        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    
    let mut mock_map = MockMap { entries: Vec::new() };
    let mut flat_map_serializer = FlatMapSerializeStruct(&mut mock_map);
    assert!(flat_map_serializer.end().is_ok());
}

#[test]
fn test_serialize_field() {
    struct MockMap {
        entries: Vec<(&'static str, &'static str)>,
    }
    
    impl SerializeMap for MockMap {
        type Error = ();
        
        fn serialize_entry(&mut self, key: &'static str, value: &impl Serialize) -> Result<(), Self::Error> {
            self.entries.push((key, value.serialize(&mut Serializer).unwrap()));
            Ok(())
        }
        
        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    
    let mut mock_map = MockMap { entries: Vec::new() };
    let mut flat_map_serializer = FlatMapSerializeStruct(&mut mock_map);
    assert!(flat_map_serializer.serialize_field("key1", &"value1").is_ok());
    assert_eq!(mock_map.entries.len(), 1);
    assert_eq!(mock_map.entries[0], ("key1", "value1"));
}

