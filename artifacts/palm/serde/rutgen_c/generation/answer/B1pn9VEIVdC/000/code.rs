// Answer 0

#[test]
fn test_serialize_entry() {
    struct MockSerializeMap {
        keys: Vec<String>,
        values: Vec<String>,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.keys.push(key.serialize().unwrap_or_else(|_| "error".to_string()));
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.values.push(value.serialize().unwrap_or_else(|_| "error".to_string()));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap {
        keys: Vec::new(),
        values: Vec::new(),
    };
    let mut serializer = FlatMapSerializeMap(&mut map);

    let key = "test_key";
    let value = "test_value";

    let result = serializer.serialize_entry(&key, &value);
    assert!(result.is_ok());
    assert_eq!(map.keys, vec!["test_key"]);
    assert_eq!(map.values, vec!["test_value"]);
}

#[test]
fn test_serialize_entry_error() {
    struct FailingSerializeMap;

    impl SerializeMap for FailingSerializeMap {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Err(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = FailingSerializeMap;
    let mut serializer = FlatMapSerializeMap(&mut map);

    let key = "failing_key";
    let value = "failing_value";

    let result = serializer.serialize_entry(&key, &value);
    assert!(result.is_err());
}

