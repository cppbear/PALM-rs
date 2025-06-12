// Answer 0

#[test]
fn test_deserialize_seq_invalid_type() {
    use serde_json::Value;
    use serde::de::{self, Visitor};

    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Err(de::Error::custom("Visited unit"))
        }
    }

    fn test_invalid_value() {
        let non_array_value = Value::String("not an array".to_string());
        let visitor = InvalidVisitor;

        let result: Result<(), _> = non_array_value.deserialize_seq(visitor).map(|_| ());
        assert!(result.is_err());
    }

    test_invalid_value();
}

