// Answer 0

#[test]
fn test_deserialize_str_with_string_value() {
    struct FakeVisitor {
        value: String,
    }

    impl<'de> Visitor<'de> for FakeVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }
    }

    let value = Value::String("test string".to_owned());
    let visitor = FakeVisitor { value: String::new() };
    let result = value.deserialize_str(visitor);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
fn test_deserialize_str_with_null_value() {
    struct FakeVisitor {
        value: String,
    }

    impl<'de> Visitor<'de> for FakeVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected a string"))
        }
    }

    let value = Value::Null;
    let visitor = FakeVisitor { value: String::new() };
    let result = value.deserialize_str(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_str_with_boolean_value() {
    struct FakeVisitor {
        value: String,
    }

    impl<'de> Visitor<'de> for FakeVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(de::Error::custom("expected a string"))
        }
    }

    let value = Value::Bool(true);
    let visitor = FakeVisitor { value: String::new() };
    let result = value.deserialize_str(visitor);
    assert!(result.is_err());
}

