// Answer 0

#[test]
fn test_tuple_variant_with_seq() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<Content<'de>>;

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(content) = seq.next_element()? {
                result.push(content);
            }
            Ok(result)
        }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };
    let result: Result<Vec<Content>, _> = deserializer.tuple_variant(2, TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Content::U8(1), Content::U8(2)]);
}

#[test]
fn test_tuple_variant_with_non_seq() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<Content<'de>>;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(de::Error::custom("should not be called"))
        }
    }

    let content = Content::U8(1);
    let deserializer = VariantRefDeserializer {
        value: Some(&content),
        err: PhantomData,
    };
    let result: Result<Vec<Content>, _> = deserializer.tuple_variant(1, TestVisitor);

    assert!(result.is_err());
}

#[test]
fn test_tuple_variant_with_none() {
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<Content<'de>>;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(de::Error::custom("should not be called"))
        }
    }

    let deserializer = VariantRefDeserializer {
        value: None,
        err: PhantomData,
    };
    let result: Result<Vec<Content>, _> = deserializer.tuple_variant(0, TestVisitor);

    assert!(result.is_err());
}

