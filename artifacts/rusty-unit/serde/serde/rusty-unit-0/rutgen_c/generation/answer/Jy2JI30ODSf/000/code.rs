// Answer 0

#[test]
fn test_visit_seq_empty() {
    struct EmptySeqVisitor;
    
    impl<'de> SeqAccess<'de> for EmptySeqVisitor {
        type Error = ();
        
        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            Ok(None)
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(0), Some(0))
        }
    }

    let visitor = EmptySeqVisitor;
    let content_visitor = ContentVisitor { value: PhantomData };
    let result = content_visitor.visit_seq(visitor);
    assert_eq!(result, Ok(Content::Seq(vec![])));
}

#[test]
fn test_visit_seq_single_element() {
    struct SingleElementSeqVisitor {
        called: bool,
    }

    impl<'de> SeqAccess<'de> for SingleElementSeqVisitor {
        type Error = ();
        
        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            if self.called {
                return Ok(None);
            }
            self.called = true;
            Ok(Some(Content::I32(42)))
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(1), Some(1))
        }
    }

    let visitor = SingleElementSeqVisitor { called: false };
    let content_visitor = ContentVisitor { value: PhantomData };
    let result = content_visitor.visit_seq(visitor);
    assert_eq!(result, Ok(Content::Seq(vec![Content::I32(42)])));
}

#[test]
fn test_visit_seq_multiple_elements() {
    struct MultipleElementsSeqVisitor {
        count: usize,
    }

    impl<'de> SeqAccess<'de> for MultipleElementsSeqVisitor {
        type Error = ();
        
        fn next_element(&mut self) -> Result<Option<Content<'de>>, Self::Error> {
            if self.count >= 3 {
                return Ok(None);
            }
            self.count += 1;
            Ok(Some(Content::I32(self.count as i32)))
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(3), Some(3))
        }
    }

    let visitor = MultipleElementsSeqVisitor { count: 0 };
    let content_visitor = ContentVisitor { value: PhantomData };
    let result = content_visitor.visit_seq(visitor);
    assert_eq!(result, Ok(Content::Seq(vec![Content::I32(1), Content::I32(2), Content::I32(3)])));
}

