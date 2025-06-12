// Answer 0

#[test]
fn test_deserialize_bool() {
    struct BoolDeserializer;

    impl Deserializer<'static> for BoolDeserializer {
        type Error = serde::de::value::Error;

        fn __deserialize_content<V>(self, _: T, visitor: V) -> Result<Content<'static>, Self::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_bool(true)
        }
    }

    let result: Result<Content<'static>, _> = Content::deserialize(BoolDeserializer);
    assert_eq!(result.unwrap(), Content::Bool(true));
}

#[test]
fn test_deserialize_u8() {
    struct U8Deserializer;

    impl Deserializer<'static> for U8Deserializer {
        type Error = serde::de::value::Error;

        fn __deserialize_content<V>(self, _: T, visitor: V) -> Result<Content<'static>, Self::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_u8(255)
        }
    }

    let result: Result<Content<'static>, _> = Content::deserialize(U8Deserializer);
    assert_eq!(result.unwrap(), Content::U8(255));
}

#[test]
fn test_deserialize_string() {
    struct StringDeserializer;

    impl Deserializer<'static> for StringDeserializer {
        type Error = serde::de::value::Error;

        fn __deserialize_content<V>(self, _: T, visitor: V) -> Result<Content<'static>, Self::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_string("Hello".to_string())
        }
    }

    let result: Result<Content<'static>, _> = Content::deserialize(StringDeserializer);
    assert_eq!(result.unwrap(), Content::String("Hello".to_string()));
}

#[test]
fn test_deserialize_seq() {
    struct SeqDeserializer;

    impl Deserializer<'static> for SeqDeserializer {
        type Error = serde::de::value::Error;

        fn __deserialize_content<V>(self, _: T, visitor: V) -> Result<Content<'static>, Self::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_seq(serde::de::value::SeqAccessDeserializer::new(vec![
                Content::U8(1),
                Content::U8(2),
            ].into_iter()))
        }
    }

    let result: Result<Content<'static>, _> = Content::deserialize(SeqDeserializer);
    assert_eq!(result.unwrap(), Content::Seq(vec![Content::U8(1), Content::U8(2)]));
}

#[should_panic]
#[test]
fn test_deserialize_fail() {
    struct FailDeserializer;

    impl Deserializer<'static> for FailDeserializer {
        type Error = serde::de::value::Error;

        fn __deserialize_content<V>(self, _: T, _visitor: V) -> Result<Content<'static>, Self::Error>
        where
            V: Visitor<'static>,
        {
            Err(serde::de::value::Error::custom("failed deserialization"))
        }
    }

    let _: Result<Content<'static>, _> = Content::deserialize(FailDeserializer);
}

