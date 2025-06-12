// Answer 0

#[test]
fn test_deserialize_any_valid_string() {
    use serde::de::{self, Visitor};
    use serde::de::value::StringDeserializer;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    let value = StringDeserializer::new("test string".to_owned());
    let result: Result<String, _> = value.deserialize_any(TestVisitor);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
#[should_panic]
fn test_deserialize_any_panic_on_empty_string() {
    use serde::de::{self, Visitor};
    use serde::de::value::StringDeserializer;

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = String;

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            panic!("This visitor should panic!");
        }
    }

    let value = StringDeserializer::new("test string".to_owned());
    let _ = value.deserialize_any(PanicVisitor);
}

