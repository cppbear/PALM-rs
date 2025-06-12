// Answer 0

#[test]
fn test_serialize_tuple_variant() {
    struct MockMap {
        key: Option<String>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key(&mut self, key: &str) -> Result<(), Self::Error> {
            self.key = Some(key.to_string());
            Ok(())
        }
        
        fn serialize_value(&mut self, _: &dyn Serialize) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { key: None };
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_tuple_variant("MyStruct", 0, "MyVariant", 0);

    assert!(result.is_ok());
    assert_eq!(map.key, Some("MyVariant".to_string()));
}

