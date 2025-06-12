// Answer 0

#[test]
fn test_deserialize_bytes_seq() {
    use serde::de::{self, Visitor};

    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = Result<Vec<u8>, de::Error>;

        fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E> {
            Err(de::Error::custom("unexpected string"))
        }

        fn visit_borrowed_str<E>(self, _value: &'de str) -> Result<Self::Value, E> {
            Err(de::Error::custom("unexpected borrowed string"))
        }

        fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E> {
            Ok(Ok(Vec::from(_value)))
        }

        fn visit_borrowed_bytes<E>(self, _value: &'de [u8]) -> Result<Self::Value, E> {
            Ok(Ok(Vec::from(_value)))
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element()? {
                result.push(value);
            }
            Ok(Ok(result))
        }
    }

    enum Content {
        Seq(Vec<u8>),
    }

    struct Deserializer {
        content: Box<Content>,
    }

    impl Deserializer {
        fn invalid_type<V>(&self, _visitor: &V) -> de::Error {
            de::Error::custom("invalid type")
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match *self.content {
                Content::Seq(ref v) => visitor.visit_seq(v),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let bytes = vec![1, 2, 3, 4];
    let deserializer = Deserializer {
        content: Box::new(Content::Seq(bytes.clone())),
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_bytes(MyVisitor);
    assert_eq!(result.unwrap().unwrap(), bytes);
}

