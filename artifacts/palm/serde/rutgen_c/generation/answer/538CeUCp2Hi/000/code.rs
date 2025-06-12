// Answer 0

#[test]
fn test_deserialize_seq_with_empty_sequence() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<()>;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error> 
        where V: SeqAccess<'de> {
            Ok(vec![])
        }

        fn visit_unit(self) -> Result<Self::Value, V::Error> {
            Ok(vec![])
        }
    }

    let content = Content::Seq(vec![]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let result: Result<Vec<()>, _> = deserializer.deserialize_seq(TestVisitor);
    assert_eq!(result.unwrap(), vec![]);
}

#[test]
fn test_deserialize_seq_with_single_element() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where V: SeqAccess<'de> {
            Ok(vec![1])
        }

        fn visit_unit(self) -> Result<Self::Value, V::Error> {
            Ok(vec![])
        }
    }

    let content = Content::Seq(vec![Content::U8(1)]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_seq(TestVisitor);
    assert_eq!(result.unwrap(), vec![1]);
}

#[test]
fn test_deserialize_seq_with_multiple_elements() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where V: SeqAccess<'de> {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element::<u8>()? {
                result.push(value);
            }
            Ok(result)
        }

        fn visit_unit(self) -> Result<Self::Value, V::Error> {
            Ok(vec![])
        }
    }

    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let result: Result<Vec<u8>, _> = deserializer.deserialize_seq(TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_invalid_content() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData::<()>,
    };

    let result: Result<(), _> = deserializer.deserialize_seq(TestVisitor);
    assert!(result.is_err());
}

