// Answer 0

#[cfg(test)]
fn test_deserialize_string() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }
    }

    let value_string = Value::String("test string".to_owned());
    let visitor = TestVisitor;

    let result: Result<String, Error> = value_string.deserialize_string(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test string");

    let value_null = Value::Null;
    let result_null: Result<String, Error> = value_null.deserialize_string(visitor);
    assert!(result_null.is_err());
}

