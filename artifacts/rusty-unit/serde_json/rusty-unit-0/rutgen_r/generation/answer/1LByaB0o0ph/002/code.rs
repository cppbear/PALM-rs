// Answer 0

#[test]
fn test_deserialize_unit_with_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null")
        }
    }

    let value = Value::Null;
    let visitor = DummyVisitor;

    let result = value.deserialize_unit(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_with_non_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct DummyVisitor;

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null")
        }
    }

    let value = Value::Bool(true); // non-null case
    let visitor = DummyVisitor;

    let result = value.deserialize_unit(visitor);
    assert!(result.is_err());
}

