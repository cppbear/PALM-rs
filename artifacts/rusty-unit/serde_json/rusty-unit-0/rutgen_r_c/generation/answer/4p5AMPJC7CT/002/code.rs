// Answer 0

#[test]
fn test_deserialize_struct_with_object() {
    // Constructing a Value::Object to test the deserialize_struct function
    let mut map = Map::new();
    let key = String::from("key");
    let value = Value::String(String::from("value"));
    map.insert(key.clone(), value);

    let object = Value::Object(map);
    let fields: &'static [&'static str] = &["key"];

    // Creating a simple Visitor to test the deserialization
    struct TestVisitor {
        result: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<String, Error>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a test visitor")
        }

        fn visit_seq<V>(self, _seq: V) -> Self::Value 
        where
            V: SeqAccess<'de>,
        {
            Ok(String::new())
        }

        fn visit_map<V>(self, mut map: V) -> Self::Value 
        where
            V: MapAccess<'de>,
        {
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                if key == "key" {
                    self.result = Some(value);
                }
            }
            Ok(self.result.clone().unwrap_or_else(|| String::new()))
        }
    }

    // Testing the deserialize_struct function
    let result = object.deserialize_struct("TestStruct", fields, TestVisitor { result: None });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "value");
}

#[test]
fn test_deserialize_struct_with_non_object() {
    // Constructing a non-Object Value (e.g., Value::Array)
    let array_value = Value::Array(vec![Value::String(String::from("value"))]);
    let fields: &'static [&'static str] = &["key"];

    // Creating a simple Visitor to test the deserialization
    struct TestVisitor {
        result: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Result<String, Error>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a test visitor")
        }

        fn visit_seq<V>(self, _seq: V) -> Self::Value 
        where
            V: SeqAccess<'de>,
        {
            Ok(String::new())
        }

        fn visit_map<V>(self, _: V) -> Self::Value 
        where
            V: MapAccess<'de>,
        {
            Err(Error)
        }
    }

    // Testing the deserialize_struct function where it should not match Object
    let result = array_value.deserialize_struct("TestStruct", fields, TestVisitor { result: None });
    assert!(result.is_err());
}

