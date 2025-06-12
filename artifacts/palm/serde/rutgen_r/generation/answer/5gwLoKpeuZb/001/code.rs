// Answer 0

#[test]
fn test_visit_seq() {
    struct TestSeqAccess;

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = serde::de::value::Error;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: serde::de::DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let seq_access = TestSeqAccess;
    let result: Result<(), <TestSeqAccess as SeqAccess>::Error> = visit_seq(seq_access);
    assert_eq!(result, Ok(()));
}

