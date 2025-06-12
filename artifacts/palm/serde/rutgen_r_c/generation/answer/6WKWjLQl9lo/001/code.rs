// Answer 0

#[test]
fn test_size_hint_none() {
    struct TestSeqAccess;

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            None
        }
    }

    let seq_access = TestSeqAccess;
    assert_eq!(seq_access.size_hint(), None);
}

