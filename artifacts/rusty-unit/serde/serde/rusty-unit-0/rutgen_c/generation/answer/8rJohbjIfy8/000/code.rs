// Answer 0

#[test]
fn test_serialize_u16() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;
        fn serialize_entry<K: Serialize, V: Serialize>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap;
    let serializer = FlatMapSerializer(&mut map);
    
    let result = serializer.serialize_u16(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u16_invalid_type() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;
        fn serialize_entry<K: Serialize, V: Serialize>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap;
    let serializer = FlatMapSerializer(&mut map);
    
    let result = serializer.serialize_u16(100);
    assert!(result.is_err());
}

