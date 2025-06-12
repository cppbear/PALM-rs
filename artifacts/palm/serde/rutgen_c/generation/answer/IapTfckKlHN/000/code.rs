// Answer 0

#[test]
fn test_deserialize_seq_with_valid_sequence() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u32>;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3])
        }
    }

    let content = Content::Seq(vec![
        Content::U32(1),
        Content::U32(2),
        Content::U32(3),
    ]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result: Result<Vec<u32>, _> = deserializer.deserialize_seq(TestVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_with_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let content = Content::Bool(true);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let result: Result<(), _> = deserializer.deserialize_seq(TestVisitor);
    assert!(result.is_err());
}

