// Answer 0

#[test]
fn test_serialize_u64() {
    struct MockMap;
    
    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_entry<K, V>(&mut self, _: K, _: V) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }

        fn serialize_key<K>(&mut self, _: K) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    #[cfg(any(feature = "std", feature = "alloc"))]
    let mut map = MockMap;

    let serializer = FlatMapSerializer(&mut map);

    let result = serializer.serialize_u64(42);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, serializer.bad_type(Unsupported::Integer));
    }
}

