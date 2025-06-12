// Answer 0

#[test]
fn test_serialize_u8() {
    struct MockMap {
        error: Option<Error>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            if self.error.is_some() {
                Err(Error)
            } else {
                Ok(())
            }
        }
    }

    let mut mock_map = MockMap { error: None };
    let serializer = FlatMapSerializer(&mut mock_map);

    let result = serializer.serialize_u8(42);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().err, Error::custom("can only flatten structs and maps (got Integer)"));
}

