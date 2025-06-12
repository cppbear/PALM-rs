// Answer 0

#[test]
fn test_deserialize_unit_struct_success() {
    struct DummyVisitor;
    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(())
        }
    }

    let input: &str = "null";
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = DummyVisitor;

    let result: Result<(), serde_json::Error> = deserializer.deserialize_unit_struct("Dummy", visitor);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_unit_struct_panic_on_unexpected_token() {
    struct DummyVisitor;
    impl<'de> serde::de::Visitor<'de> for DummyVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(())
        }
    }

    let input: &str = "{ }"; // Not a unit
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let visitor = DummyVisitor;

    let _result: Result<(), serde_json::Error> = deserializer.deserialize_unit_struct("Dummy", visitor);
}

