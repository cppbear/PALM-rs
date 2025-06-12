// Answer 0

#[test]
fn test_visit_seq_zero_length() {
    struct EmptySeq;
    impl<'de> SeqAccess<'de> for EmptySeq {
        type Error = ();
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error> {
            Ok(None)
        }
        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let seq = EmptySeq;
    let result: Result<[(); 0], ()> = ArrayVisitor::<[(); 0]>::default().visit_seq(seq);
}

