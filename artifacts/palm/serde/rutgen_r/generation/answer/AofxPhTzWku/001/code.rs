// Answer 0

#[test]
fn test_deserialize_seq_with_err_visitation() {
    struct VisitorErr;

    impl<'de> serde::de::Visitor<'de> for VisitorErr {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(serde::de::Error::custom("visit_seq error"))
        }

        fn visiting(self, _: &str) -> Self {
            self
        }
    }

    struct DeserializerErr;

    impl DeserializerErr {
        fn end(&mut self) -> Result<(), serde::de::Error> {
            Ok(())
        }
    }

    impl<'de> serde::de::Deserializer<'de> for DeserializerErr {
        type Error = serde::de::Error;

        fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let value = visitor.visit_seq(&mut self)?;
            self.end()?;
            Ok(value)
        }

        fn deserialize_any<V>(self, _: V) -> Result<Self::Value, Self::Error> {
            Err(serde::de::Error::custom("deserialize_any error"))
        }
    }

    let deserializer = DeserializerErr;
    let visitor = VisitorErr;
    let result: Result<(), serde::de::Error> = deserializer.deserialize_seq(visitor);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "visit_seq error");
}

