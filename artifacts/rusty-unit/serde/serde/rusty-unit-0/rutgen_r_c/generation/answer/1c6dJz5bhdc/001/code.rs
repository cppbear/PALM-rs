// Answer 0

#[test]
fn test_bad_type_boolean() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Boolean);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Boolean)"));
}

#[test]
fn test_bad_type_integer() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Integer);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Integer)"));
}

#[test]
fn test_bad_type_float() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Float);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Float)"));
}

#[test]
fn test_bad_type_char() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Char);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Char)"));
}

#[test]
fn test_bad_type_string() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::String);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got String)"));
}

#[test]
fn test_bad_type_byte_array() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::ByteArray);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got ByteArray)"));
}

#[test]
fn test_bad_type_optional() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Optional);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Optional)"));
}

#[test]
fn test_bad_type_sequence() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Sequence);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Sequence)"));
}

#[test]
fn test_bad_type_tuple() {
    struct MockSerializeMap;
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = String;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<(), Self::Error> {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mock_serializer = FlatMapSerializer(&mut MockSerializeMap);
    let result = FlatMapSerializer::<MockSerializeMap>::bad_type(Unsupported::Tuple);
    assert_eq!(result, ser::Error::custom("can only flatten structs and maps (got Tuple)"));
}

