// Answer 0

#[test]
fn test_deserialize_unit_struct() {
    use serde::de::{Visitor, Deserialize};
    use serde_json::de::Deserializer;
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit struct")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(())
        }
    }

    let json = r#""#; // empty JSON representing unit value
    let deserializer = &mut Deserializer::from_str(json);
    let visitor = TestVisitor;

    let result: Result<(), Error> = deserializer.deserialize_unit_struct("Test", visitor);
    assert!(result.is_ok());
}

