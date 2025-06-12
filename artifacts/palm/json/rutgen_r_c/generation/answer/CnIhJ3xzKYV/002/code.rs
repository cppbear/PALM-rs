// Answer 0

fn test_deserialize_any_success() {
    struct TestVisitor {
        expected_len: usize,
        visited: usize,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<String>;

        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some((key, _)) = visitor.next_entry::<String, Value>()? {
                self.visited += 1;
                result.push(key);
            }
            Ok(result)
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Null);
    map.insert("key2".to_owned(), Value::Null);

    let visitor = TestVisitor {
        expected_len: map.len(),
        visited: 0,
    };

    let result: Result<Vec<String>, Error> = (&map).deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

fn test_deserialize_any_failure() {
    struct FailVisitor {}

    impl<'de> Visitor<'de> for FailVisitor {
        type Value = ();

        fn visit_map<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            Err(de::Error::custom("visitor fail"))
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Null);
    
    let visitor = FailVisitor {};
    
    let result: Result<(), Error> = (&map).deserialize_any(visitor);
    assert!(result.is_err());
}

fn test_deserialize_any_remaining_elements() {
    struct TestVisitor {
        visited: usize,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            visitor.next_entry::<String, Value>()?;
            visitor.next_entry::<String, Value>()?; // Two entries for testing.
            Ok(())
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Null);
    map.insert("key2".to_owned(), Value::Null);

    let visitor = TestVisitor { visited: 0 };
    
    let result: Result<(), Error> = (&map).deserialize_any(visitor);
    assert!(result.is_err());
}

