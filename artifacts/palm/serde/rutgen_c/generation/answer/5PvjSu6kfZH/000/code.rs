// Answer 0

#[test]
fn test_serialize_bytes() {
    struct SimpleMapSerializer;

    impl SerializeMap for SimpleMapSerializer {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn serialize_key<K>(&mut self, _: &K) -> Result<(), Self::Error>
        where
            K: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut serializer = FlatMapSerializer(&mut SimpleMapSerializer);

    let result = serializer.serialize_bytes(&[1, 2, 3]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().err, ErrorImpl::custom("can only flatten structs and maps (got ByteArray)"));
}

