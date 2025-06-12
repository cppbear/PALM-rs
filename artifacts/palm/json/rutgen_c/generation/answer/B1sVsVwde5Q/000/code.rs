// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    struct TestVisitor;
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of strings")
        }

        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<String>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Array(vec![
        Value::String("value1".to_string()),
        Value::String("value2".to_string()),
    ]);

    let result: Result<Vec<String>, Error> = value.deserialize_struct("TestStruct", &[]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!["value1".to_string(), "value2".to_string()]);
}

#[test]
fn test_deserialize_struct_with_object() {
    struct TestVisitor;
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<(String, String)>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of string pairs")
        }

        fn visit_map<V>(self, map: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            let mut entries = Vec::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                entries.push((key, value));
            }
            Ok(entries)
        }
    }

    let mut object_map = Map::new();
    object_map.insert("key1".to_string(), Value::String("value1".to_string()));
    object_map.insert("key2".to_string(), Value::String("value2".to_string()));
    let value = Value::Object(object_map);

    let result: Result<Vec<(String, String)>, Error> = value.deserialize_struct("TestStruct", &[]);
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], ("key1".to_string(), "value1".to_string()));
    assert_eq!(entries[1], ("key2".to_string(), "value2".to_string()));
}

#[test]
fn test_deserialize_struct_invalid_type() {
    struct TestVisitor;
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object or an array")
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let value = Value::Bool(true);

    let result: Result<(), Error> = value.deserialize_struct("TestStruct", &[]);
    assert!(result.is_err());
}

