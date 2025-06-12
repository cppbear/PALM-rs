// Answer 0

#[test]
fn test_deserialize_identifier_bytes() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_bytes(self, value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Ok(value.to_vec())
        }

        fn visit_borrowed_bytes(self, _value: &'de [u8]) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("borrowed bytes not expected"))
        }

        fn visit_str(self, _value: &str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("string not expected"))
        }

        fn visit_borrowed_str(self, _value: &'de str) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("borrowed string not expected"))
        }

        fn visit_u8(self, _value: u8) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("u8 not expected"))
        }

        fn visit_u64(self, _value: u64) -> Result<Self::Value, serde::de::Error> {
            Err(serde::de::Error::custom("u64 not expected"))
        }
    }

    struct ContentWrapper {
        content: Content,
    }

    impl ContentWrapper {
        fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
            serde::de::Error::custom("invalid type")
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Bytes(ref v) => visitor.visit_borrowed_bytes(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Bytes(&'static [u8]),
        // other variants can be here, but omitted for this test
    }

    let content = Content::Bytes(&[1, 2, 3, 4]);
    let wrapper = ContentWrapper { content };

    let result = wrapper.deserialize_identifier(TestVisitor);
    assert_eq!(result, Ok(vec![1, 2, 3, 4]));
}

