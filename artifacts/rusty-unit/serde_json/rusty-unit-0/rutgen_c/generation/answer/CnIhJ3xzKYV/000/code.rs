// Answer 0

#[test]
fn test_deserialize_any_empty_map() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an empty map")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let map = Map::new();
    let result: Result<(), Error> = (&map).deserialize_any(TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_non_empty_map() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a non-empty map")
        }

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));

    let result: Result<(), Error> = (&map).deserialize_any(TestVisitor);
    assert!(result.is_err());
}

