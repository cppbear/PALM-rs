// Answer 0

#[test]
fn test_serialize_map_with_some() {
    struct MockMap {
        serialized: Vec<(String, ())>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;
        
        fn serialize_entry<K: Serialize, V: Serialize>(
            &mut self,
            key: &K,
            value: &V,
        ) -> Result<(), Self::Error> {
            let key_string = format!("{:?}", key);
            self.serialized.push((key_string, ()));
            Ok(())
        }

        fn serialize_key<K: Serialize>(&mut self, key: &K) -> Result<(), Self::Error> {
            self.serialize_entry(key, &())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMap { serialized: Vec::new() };
    
    let serializer = FlatMapSerializer(&mut mock_map);
    let result = serializer.serialize_map(Some(1));
    
    assert!(result.is_ok());
}

#[test]
fn test_serialize_map_with_none() {
    struct MockMap {
        serialized: Vec<(String, ())>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;
        
        fn serialize_entry<K: Serialize, V: Serialize>(
            &mut self,
            key: &K,
            value: &V,
        ) -> Result<(), Self::Error> {
            let key_string = format!("{:?}", key);
            self.serialized.push((key_string, ()));
            Ok(())
        }

        fn serialize_key<K: Serialize>(&mut self, key: &K) -> Result<(), Self::Error> {
            self.serialize_entry(key, &())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMap { serialized: Vec::new() };
    
    let serializer = FlatMapSerializer(&mut mock_map);
    let result = serializer.serialize_map(None);
    
    assert!(result.is_ok());
}

