// Answer 0

#[test]
fn test_serialize_tuple_struct() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Error = ();

        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize 
        {
            Ok(())
        }
        
        fn serialize_key<K>(&mut self, _key: K) -> Result<(), Self::Error> 
        where 
            K: Serialize 
        {
            Ok(())
        }
    }

    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_tuple_struct("TestStruct", 2);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "can only flatten structs and maps (got Tuple)");
    }
}

