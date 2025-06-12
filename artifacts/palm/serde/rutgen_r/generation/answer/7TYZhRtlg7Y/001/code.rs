// Answer 0

#[test]
fn test_deserialize_struct_invalid_type() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any valid type")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(())
        }
    }

    struct TestDeserializer {
        content: Content,
    }

    impl TestDeserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> String {
            "Invalid type".to_string()
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, String>
        where
            V: serde::de::Visitor<'de>,
        {
            match &self.content {
                Content::Seq(_) => Err(self.invalid_type(&visitor)),
                Content::Map(_) => Err(self.invalid_type(&visitor)),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Seq(Vec<u8>),
        Map(std::collections::HashMap<String, String>),
        Other,
    }

    let deserializer = TestDeserializer {
        content: Content::Other,
    };

    let result: Result<(), String> = deserializer.deserialize_struct("test", &["field1"], TestVisitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

