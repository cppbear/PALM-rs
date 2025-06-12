// Answer 0

#[test]
fn test_visit_seq_with_empty_sequence() {
    struct EmptySeqAccess;

    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(
            self,
            _seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };

    let result = visitor.visit_seq(EmptySeqAccess);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), TagOrContent::Content(Content::Seq(ref seq)) if seq.is_empty()));
}

#[test]
fn test_visit_seq_with_single_element() {
    struct SingleElementSeqAccess;

    impl<'de> SeqAccess<'de> for SingleElementSeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(
            self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if seed.deserialize(serde::de::value::BorrowedStr::new("value")).is_ok() {
                return Ok(Some(seed.deserialize(serde::de::value::BorrowedStr::new("value")).unwrap().1));
            }
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(1)
        }
    }

    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };

    let result = visitor.visit_seq(SingleElementSeqAccess);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), TagOrContent::Content(Content::Seq(ref seq)) if seq.len() == 1));
}

#[test]
fn test_visit_seq_with_multiple_elements() {
    struct MultipleElementsSeqAccess {
        count: usize,
    }

    impl<'de> SeqAccess<'de> for MultipleElementsSeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(
            mut self,
            seed: T,
        ) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.count > 0 {
                self.count -= 1;
                return Ok(Some(seed.deserialize(serde::de::value::BorrowedStr::new("value")).unwrap().1));
            }
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.count)
        }
    }

    let visitor = TagOrContentVisitor { name: "test", value: PhantomData };

    let mut seq_access = MultipleElementsSeqAccess { count: 3 };
    let result = visitor.visit_seq(seq_access);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), TagOrContent::Content(Content::Seq(ref seq)) if seq.len() == 3));
}

