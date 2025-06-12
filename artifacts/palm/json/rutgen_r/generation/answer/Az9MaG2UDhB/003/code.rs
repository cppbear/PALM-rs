// Answer 0

#[test]
fn test_struct_variant_none() {
    use serde::de::{self, Visitor, Unexpected};
    use serde_json::Value;
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct variant")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    struct TestStruct {
        value: Option<Value>,
    }

    let test_instance = TestStruct { value: None };

    let result: Result<(), Error> = test_instance.struct_variant(&[], TestVisitor).map_err(|e| e);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().kind(), Error::invalid_type(Unexpected::UnitVariant, &"struct variant").kind());
}

