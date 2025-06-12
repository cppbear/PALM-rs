// Answer 0

#[test]
fn test_deserialize_seq_with_valid_content() {
    use serde::de::Visitor;

    struct TestVisitor {
        value: Option<Vec<u8>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn visit_seq<V>(self, mut _seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut data = Vec::new();
            while let Some(value) = _seq.next_element::<u8>()? {
                data.push(value);
            }
            Ok(data)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of u8 values")
        }
    }

    let seq_content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer = ContentDeserializer {
        content: seq_content,
        err: std::marker::PhantomData,
    };

    let result = deserializer.deserialize_seq(TestVisitor { value: None });
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_seq_with_non_sequence_content() {
    use serde::de::Visitor;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a unit")
        }
    }

    let non_seq_content = Content::Bool(true); // this is not a sequence
    let deserializer = ContentDeserializer {
        content: non_seq_content,
        err: std::marker::PhantomData,
    };

    let _ = deserializer.deserialize_seq(TestVisitor);
}

