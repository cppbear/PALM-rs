// Answer 0

#[test]
fn test_end_valid_case() {
    struct DummyMap;

    impl SerializeMap for DummyMap {
        type Error = Error;
        
        fn serialize_entry<K: ?Sized + Serialize>(&mut self, _key: &K, _value: &impl Serialize) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = DummyMap;
    let mut serializer = FlatMapSerializeStruct(&mut map);
    let result = serializer.end();
}

#[test]
fn test_end_after_serializing_field() {
    struct AnotherDummyMap;

    impl SerializeMap for AnotherDummyMap {
        type Error = Error;
        
        fn serialize_entry<K: ?Sized + Serialize>(&mut self, _key: &K, _value: &impl Serialize) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = AnotherDummyMap;
    let mut serializer = FlatMapSerializeStruct(&mut map);
    let _ = serializer.serialize_field("valid_key", &"valid_value");
    let result = serializer.end();
}

#[test]
fn test_end_multiple_fields() {
    struct MultiFieldDummyMap;

    impl SerializeMap for MultiFieldDummyMap {
        type Error = Error;

        fn serialize_entry<K: ?Sized + Serialize>(&mut self, _key: &K, _value: &impl Serialize) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = MultiFieldDummyMap;
    let mut serializer = FlatMapSerializeStruct(&mut map);
    let _ = serializer.serialize_field("key1", &"value1");
    let _ = serializer.serialize_field("key2", &42);
    let _ = serializer.serialize_field("key3", &true);
    let result = serializer.end();
}

#[test]
#[should_panic]
fn test_end_invalid_case() {
    struct InvalidDummyMap;

    impl SerializeMap for InvalidDummyMap {
        type Error = Error;

        fn serialize_entry<K: ?Sized + Serialize>(&mut self, _key: &K, _value: &impl Serialize) -> Result<(), Self::Error> {
            Err(Error)
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = InvalidDummyMap;
    let mut serializer = FlatMapSerializeStruct(&mut map);
    let _ = serializer.serialize_field("key", &"value");
    let result = serializer.end(); // should panic due to entry serialization failure
}

