// Answer 0

#[test]
fn test_deserialize_any_success() {
    struct MockVisitor {
        done: bool,
    }

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            if self.done {
                Ok("mock_value".to_string())
            } else {
                Err(de::Error::custom("visit_map error"))
            }
        }

        // Additional methods for Visitor would need to be implemented here if used
    }

    struct TestMapDeserializer {
        iter: std::iter::Empty<()>,
        count: usize,
    }

    impl<'de> de::Deserializer<'de> for TestMapDeserializer {
        type Error = Box<str>;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let result = visitor.visit_map(&mut self);
            self.end()?;
            result
        }

        fn end(self) -> Result<(), Self::Error> {
            Err("end error".into())
        }

        // Other methods to satisfy Deserializer trait would need to be stubbed
    }

    let deserializer = TestMapDeserializer {
        iter: std::iter::empty(),
        count: 0,
    };

    let visitor = MockVisitor { done: false };
    let result = deserializer.deserialize_any(visitor);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref(), "end error");
}

#[test]
fn test_deserialize_any_error_on_end() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestMapDeserializer {
        iter: std::iter::Empty<()>,
        count: usize,
    }

    impl<'de> de::Deserializer<'de> for TestMapDeserializer {
        type Error = Box<str>;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let _ = visitor.visit_map(&mut self);
            self.end() // This should trigger an error
        }

        fn end(self) -> Result<(), Self::Error> {
            Err("end error".into())
        }

        // Other methods to satisfy Deserializer trait would need to be stubbed
    }

    let deserializer = TestMapDeserializer {
        iter: std::iter::empty(),
        count: 0,
    };

    let visitor = MockVisitor;
    let result = deserializer.deserialize_any(visitor);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref(), "end error");
}

