// Answer 0

#[test]
fn test_serialize_str() {
    struct TestMap;

    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = &'static str;

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error> 
        where K: Serialize, V: Serialize {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut test_map = TestMap;
    let serializer = FlatMapSerializer(&mut test_map);
    let result = serializer.serialize_str("test");

    assert_eq!(result, Err("can only flatten structs and maps (got String)"));
}

