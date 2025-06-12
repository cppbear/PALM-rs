// Answer 0

#[test]
fn test_deserialize_char_with_content_char() {
    struct TestVisitor;
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = char;

        fn visit_char<E>(self, value: char) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected char, found str"))
        }

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
            Err(serde::de::Error::custom("expected char, found borrowed str"))
        }
    }

    struct TestContent {
        content: Content,
    }

    enum Content {
        Char(char),
        String(String),
        Str(&'static str),
        Invalid,
    }

    impl TestContent {
        fn invalid_type<V>(&self, _: &V) -> serde::de::Error {
            serde::de::Error::custom("invalid type")
        }

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Char(v) => visitor.visit_char(v),
                Content::String(ref v) => visitor.visit_str(v),
                Content::Str(v) => visitor.visit_borrowed_str(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }
    
    let content = TestContent { content: Content::Char('a') };
    let visitor = TestVisitor;

    let result = content.deserialize_char(visitor).unwrap();
    assert_eq!(result, 'a');
}

