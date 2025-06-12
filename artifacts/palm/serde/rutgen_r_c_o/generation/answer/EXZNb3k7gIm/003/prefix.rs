// Answer 0

#[test]
fn test_visit_seq_with_ok_elements_followed_by_err() {
    struct MockSeqAccess {
        calls: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        fn next_element<D>(&mut self) -> Result<Option<IgnoredAny>, D::Error>
        where
            D: Deserializer<'de>,
        {
            if self.calls < 2 {
                self.calls += 1;
                Ok(Some(IgnoredAny))
            } else {
                Err(/* some error here */)
            }
        }
    }

    let mut seq = MockSeqAccess { calls: 0 };
    let visitor = IgnoredAny;

    let _ = visitor.visit_seq(&mut seq);
}

#[test]
fn test_visit_seq_with_multiple_ok_elements() {
    struct MockSeqAccess {
        calls: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        fn next_element<D>(&mut self) -> Result<Option<IgnoredAny>, D::Error>
        where
            D: Deserializer<'de>,
        {
            if self.calls < 3 {
                self.calls += 1;
                Ok(Some(IgnoredAny))
            } else {
                Ok(None)
            }
        }
    }

    let mut seq = MockSeqAccess { calls: 0 };
    let visitor = IgnoredAny;

    let _ = visitor.visit_seq(&mut seq);
}

#[test]
#[should_panic]
fn test_visit_seq_with_err_on_first_element() {
    struct MockSeqAccess;

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        fn next_element<D>(&mut self) -> Result<Option<IgnoredAny>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Err(/* some error here */)
        }
    }

    let mut seq = MockSeqAccess;
    let visitor = IgnoredAny;

    let _ = visitor.visit_seq(&mut seq);
}

#[test]
fn test_visit_seq_with_err_on_last_element() {
    struct MockSeqAccess {
        calls: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        fn next_element<D>(&mut self) -> Result<Option<IgnoredAny>, D::Error>
        where
            D: Deserializer<'de>,
        {
            if self.calls < 2 {
                self.calls += 1;
                Ok(Some(IgnoredAny))
            } else {
                Err(/* some error here */)
            }
        }
    }

    let mut seq = MockSeqAccess { calls: 0 };
    let visitor = IgnoredAny;

    let _ = visitor.visit_seq(&mut seq);
}

