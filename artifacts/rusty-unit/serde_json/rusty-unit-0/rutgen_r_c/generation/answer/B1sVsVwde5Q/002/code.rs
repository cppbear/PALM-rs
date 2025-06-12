// Answer 0

#[test]
fn test_deserialize_struct_object() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::Object(Map {
        map: Default::default(),
    });

    let result = value.deserialize_struct("TestStruct", &["field1", "field2"], MockVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_struct_array() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::Array(vec![Value::Bool(true)]);

    let result = value.deserialize_struct("TestStruct", &["field1", "field2"], MockVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_struct_invalid_type() {
    struct MockVisitor;

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object")
        }

        fn visit_map<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: MapAccess<'de>,
        {
            Ok(())
        }
    }

    let value = Value::Null;

    let result = value.deserialize_struct("TestStruct", &["field1", "field2"], MockVisitor);
    assert!(result.is_err());
}

