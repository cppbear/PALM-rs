// Answer 0

#[test]
fn test_deserialize_ignored_any_error_condition() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value> {
            Ok(())
        }
    }

    struct MockDeserializer {
        should_error: bool,
    }

    impl MockDeserializer {
        fn ignore_value(&self) -> Result<()> {
            if self.should_error {
                Err(serde_json::de::Error::custom("mock error"))
            } else {
                Ok(())
            }
        }
    }

    fn deserialize_ignored_any<V>(self: &MockDeserializer, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.ignore_value()?;
        visitor.visit_unit()
    }

    let deserializer = MockDeserializer { should_error: true };
    let result = deserialize_ignored_any(&deserializer, MockVisitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "mock error");
}

