// Answer 0

#[test]
fn test_visit_seq_with_some_elements() {
    struct TestSeqAccess {
        elements: Vec<Content<'static>>,
        current: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = std::convert::Infallible; // Choose an infallible error for the sake of simplicity

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(self.elements.len()), Some(self.elements.len()))
        }

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            if self.current < self.elements.len() {
                let element = self.elements[self.current].clone();
                self.current += 1;
                Ok(Some(element))
            } else {
                Ok(None)
            }
        }
    }

    let visitor = TestSeqAccess {
        elements: vec![
            Content::I32(10),
            Content::String("test".to_string()),
            Content::F64(3.14),
        ],
        current: 0,
    };

    let visitor_instance = ContentVisitor { value: std::marker::PhantomData };
    let result = visitor_instance.visit_seq(visitor).unwrap();

    match result {
        Content::Seq(seq) => {
            assert_eq!(seq.len(), 3);
            assert_eq!(seq[0], Content::I32(10));
            assert_eq!(seq[1], Content::String("test".to_string()));
            assert_eq!(seq[2], Content::F64(3.14));
        }
        _ => panic!("Expected a Content::Seq"),
    }
}

#[test]
fn test_visit_seq_with_empty() {
    struct EmptySeqAccess;

    impl<'de> SeqAccess<'de> for EmptySeqAccess {
        type Error = std::convert::Infallible;

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(0), Some(0))
        }

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            Ok(None)
        }
    }

    let visitor_instance = ContentVisitor { value: std::marker::PhantomData };
    let result = visitor_instance.visit_seq(EmptySeqAccess).unwrap();

    match result {
        Content::Seq(seq) => {
            assert_eq!(seq.len(), 0);
        }
        _ => panic!("Expected a Content::Seq with no elements"),
    }
}

#[test]
fn test_visit_seq_with_error() {
    struct ErrorSeqAccess {
        current: usize,
    }

    impl<'de> SeqAccess<'de> for ErrorSeqAccess {
        type Error = std::convert::Infallible;

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(1), Some(1))
        }

        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            if self.current == 0 {
                self.current += 1;
                Ok(Some(Content::I32(10)))
            } else {
                Err(std::convert::Infallible) // Trigger an error after the first element
            }
        }
    }

    let visitor_instance = ContentVisitor { value: std::marker::PhantomData };
    let result = visitor_instance.visit_seq(ErrorSeqAccess { current: 0 });

    assert!(result.is_err());
}

