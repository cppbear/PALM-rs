// Answer 0

#[test]
fn test_serialize_tuple() {
    struct MockSerializeMap;

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();
        fn serialize_key<K: ?Sized + Serialize>(&mut self, _: &K) -> Result<(), Self::Error> {
            Ok(())
        }
        fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
            &mut self,
            _: &K,
            _: &V,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockSerializeMap;

    let serializer = FlatMapSerializer(&mut mock_map);
    let result = serializer.serialize_tuple(1);

    assert_eq!(result.is_err(), true);
    if let Err(err) = result {
        // Optionally check that the error matches the expected bad type.
        assert_eq!(err.to_string(), "can only flatten structs and maps (got Tuple)");
    }
}

