// Answer 0

#[test]
fn test_deserialize_unit_with_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct NullVisitor;

    impl<'de> Visitor<'de> for NullVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null")
        }

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }
    }

    let value = Value::Null;
    let visitor = NullVisitor;

    let result = value.deserialize_unit(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

#[test]
#[should_panic]
fn test_deserialize_unit_with_non_null() {
    use serde::de::{self, Visitor};
    use serde_json::Value;

    struct NonNullVisitor;

    impl<'de> Visitor<'de> for NonNullVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("non-null")
        }

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }
    }

    let value = Value::String(String::from("not null"));
    let visitor = NonNullVisitor;

    // This will panic because the value is not null.
    let _ = value.deserialize_unit(visitor);
}

