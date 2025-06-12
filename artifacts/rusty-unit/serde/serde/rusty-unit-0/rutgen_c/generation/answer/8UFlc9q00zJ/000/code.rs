// Answer 0

#[test]
fn test_serialize_struct_success() {
    struct MockMap {
        entries: Vec<(String, String)>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMap { entries: vec![] };
    let serializer = FlatMapSerializer(&mut mock_map);
    
    let result = serializer.serialize_struct("test_struct", 0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_struct_fail() {
    struct MockMapFail {
        entries: Vec<(String, String)>,
    }

    impl SerializeMap for MockMapFail {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMapFail { entries: vec![] };
    let serializer = FlatMapSerializer(&mut mock_map);
    
    let result = serializer.serialize_struct("test_struct", 0);
    assert!(result.is_err());
}

