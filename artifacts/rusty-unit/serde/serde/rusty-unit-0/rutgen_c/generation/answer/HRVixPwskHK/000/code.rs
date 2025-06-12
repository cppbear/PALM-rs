// Answer 0

#[test]
fn test_deserialize_tuple_valid_length() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple of length 2")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let _first: () = seq.next_element()?.ok_or_else(|| {
                de::Error::invalid_length(0, &ExpectedInSeq(2))
            })?;
            let _second: () = seq.next_element()?.ok_or_else(|| {
                de::Error::invalid_length(1, &ExpectedInSeq(2))
            })?;
            Ok(())
        }
    }

    let deserializer = PairDeserializer((), (), PhantomData::<()>);
    let result = deserializer.deserialize_tuple(2, VisitorImpl);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_tuple_invalid_length() {
    struct VisitorImpl;

    impl<'de> de::Visitor<'de> for VisitorImpl {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple of length 2")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let deserializer = PairDeserializer((), (), PhantomData::<()>);
    let result = deserializer.deserialize_tuple(3, VisitorImpl);
    assert!(result.is_err());
}

