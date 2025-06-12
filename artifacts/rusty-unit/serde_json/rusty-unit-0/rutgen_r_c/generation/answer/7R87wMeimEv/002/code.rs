// Answer 0

fn test_deserialize_any_ok() {
    struct MockVisitor {
        value: Result<(), Error>,
    }
    
    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Vec<(&'de str, &'de Value)>;

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Self::Error>
        where
            M: MapAccess<'de>,
        {
            self.value.map(|_| vec![("key", &Value::Null)])
        }
    }

    let mut map = Map::new();
    map.insert("key".to_string(), Value::Null);
    let visitor = MockVisitor { value: Ok(()) };
    
    let result: Result<Vec<(&str, &Value)>, Error> = map.deserialize_any(visitor);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.len(), 1);
}

fn test_deserialize_any_with_remaining_elements() {
    struct MockVisitor {
        call_count: usize,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Self::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Null); // Additional element to cause remaining > 0

    let visitor = MockVisitor { call_count: 0 };

    let result: Result<(), Error> = map.deserialize_any(visitor);
    assert!(result.is_err());
}

fn test_deserialize_any_empty_map() {
    struct MockVisitor {}

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, Self::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let map = Map::new(); // Empty map
    let visitor = MockVisitor {};

    let result: Result<(), Error> = map.deserialize_any(visitor);
    assert!(result.is_ok());
}

