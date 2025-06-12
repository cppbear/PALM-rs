// Answer 0

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor {
        called: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_some<E>(self, _: E) -> Result<Self::Value, Error>
        where
            E: serde::de::Deserializer<'de>,
        {
            Ok(())
        }
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an option")
        }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("test_key"),
    };

    let visitor = TestVisitor { called: false };
    let result = deserializer.deserialize_option(visitor);

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_some<E>(self, _: E) -> Result<Self::Value, Error>
        where
            E: serde::de::Deserializer<'de>,
        {
            Err(Error {})
        }

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an option")
        }
    }

    let deserializer = MapKeyDeserializer {
        key: Cow::Borrowed("test_key"),
    };

    let visitor = TestVisitor;
    let result = deserializer.deserialize_option(visitor);
    
    assert!(result.is_err());
}

