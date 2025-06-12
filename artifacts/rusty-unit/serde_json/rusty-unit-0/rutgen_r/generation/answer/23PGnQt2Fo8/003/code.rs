// Answer 0

#[test]
fn test_deserialize_enum_iter_empty() {
    use serde::de::{self, Visitor, Unexpected};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_enum<E>(self, _deserializer: E) -> Result<Self::Value, E::Error>
        where
            E: serde::de::EnumAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestDeserializer {
        data: Vec<(String, String)>,
    }

    impl TestDeserializer {
        fn into_iter(self) -> std::iter::IntoIter<(String, String)> {
            self.data.into_iter()
        }
    }

    let deserializer = TestDeserializer { data: vec![] };
    let result: Result<(), _> = deserializer.deserialize_enum("TestEnum", &["Variant1", "Variant2"], TestVisitor);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), de::Error::invalid_value(Unexpected::Map, &"map with a single key"));
}

