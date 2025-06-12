// Answer 0

#[test]
fn test_deserialize_seq_error_case() {
    use serde::de::{self, Visitor};
    use std::marker::PhantomData;

    struct FakeVisitor {
        should_error: bool,
    }

    impl<'de> Visitor<'de> for FakeVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            if self.should_error {
                Err(de::Error::custom("forced error"))
            } else {
                Ok(())
            }
        }
    }

    struct FakeDeserializer;

    impl FakeDeserializer {
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            let mut pair_visitor = PairVisitor(Some(0), Some(0), PhantomData);
            let pair = visitor.visit_seq(&mut pair_visitor);
            if pair_visitor.1.is_none() {
                pair
            } else {
                let remaining = pair_visitor.size_hint().unwrap();
                Err(de::Error::invalid_length(2, &ExpectedInSeq(2 - remaining)))
            }
        }
    }

    let deserializer = FakeDeserializer;
    let visitor = FakeVisitor { should_error: true };
    let result = deserializer.deserialize_seq(visitor);
    assert!(result.is_err());
}

