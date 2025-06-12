// Answer 0

#[test]
fn test_serialize_tuple_struct() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K: Serialize>(&mut self, _key: K, _value: &()) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn serialize_key<K: Serialize>(&mut self, _: K) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    {
        let mut mock_map = MockSerializeMap;
        let serializer = FlatMapSerializer(&mut mock_map);

        let result = serializer.serialize_tuple_struct("TestStruct", 2);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, MockSerializeMap::bad_type(Unsupported::TupleStruct));
        }
    }
}

