// Answer 0

#[test]
fn test_serialize_unit_struct_success() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

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
    let result = serializer.serialize_unit_struct("MyStruct");
    assert_eq!(result, Ok(()));
}

