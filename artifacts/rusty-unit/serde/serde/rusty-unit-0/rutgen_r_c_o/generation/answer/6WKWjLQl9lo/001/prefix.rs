// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestSeq;

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = Error;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Err(Error)
        }
    }

    let mut seq = TestSeq;
    let result = seq.size_hint();
}

#[test]
fn test_size_hint_empty() {
    struct EmptySeq;

    impl<'de> SeqAccess<'de> for EmptySeq {
        type Error = Error;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Ok(None)
        }
    }

    let mut seq = EmptySeq;
    let result = seq.size_hint();
}

