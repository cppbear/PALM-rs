// Answer 0

#[test]
fn test_deserialize_seq_with_sequence_content() {
    use crate::de::Visitor;
    use crate::de::Error;
    use crate::de::value::SeqDeserializer;

    struct TestVisitor {
        values: Vec<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, visitor: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = visitor.next_element::<u8>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let content = Content::Seq(vec![
        Content::U8(1),
        Content::U8(2),
        Content::U8(3),
    ]);

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor { values: Vec::new() };
    let result: Vec<u8> = deserializer.deserialize_seq(visitor).unwrap();

    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_with_non_sequence_content() {
    use crate::de::Visitor;
    use crate::de::Error;

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

    let content = Content::U8(42); // Not a Seq

    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };

    let visitor = TestVisitor;

    let result = deserializer.deserialize_seq(visitor);

    assert!(result.is_err()); // Expecting an error
}

