// Answer 0

#[test]
fn test_deserialize_unit_with_non_null_value() {
    use serde::de::Deserializer;
    use serde::de::Visitor;
    use serde::de::{self, Error as SerdeError};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("null")
        }

        fn visit_unit(self) -> Result<Self::Value, SerdeError> {
            Err(SerdeError::custom("visit_unit should not be called"))
        }
    }

    let value = Value::Bool(true); // This matches Value::Null == false
    let visitor = TestVisitor;

    let result: Result<(), Error> = value.deserialize_unit(visitor);

    assert!(result.is_err());
}

