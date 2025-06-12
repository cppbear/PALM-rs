// Answer 0

#[test]
fn test_serialize_struct() {
    struct MockMap;
    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key(&mut self, _: &str) -> Result<(), Self::Error> {
            Ok(())
        }

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error>
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

    let mut mock_map = MockMap;

    let serializer = FlatMapSerializer(&mut mock_map);
    let result = serializer.serialize_struct("TestStruct", 0);

    assert!(result.is_ok());
    assert!(result.unwrap().0.serialize_entry("key", &()).is_ok());
}

