// Answer 0

#[test]
fn test_visit_seq_empty() {
    struct TestAccess {
        elements: Vec<IgnoredAny>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<IgnoredAny>, Self::Error> {
            if self.index < self.elements.len() {
                let element = self.elements[self.index];
                self.index += 1;
                Ok(Some(element))
            } else {
                Ok(None)
            }
        }
    }

    let access = TestAccess { elements: vec![], index: 0 };
    let result = visit_seq(access);
    assert_eq!(result.unwrap(), IgnoredAny);
}

#[test]
fn test_visit_seq_with_elements() {
    struct TestAccess {
        elements: Vec<IgnoredAny>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestAccess {
        type Error = ();

        fn next_element<T>(&mut self) -> Result<Option<IgnoredAny>, Self::Error> {
            if self.index < self.elements.len() {
                let element = self.elements[self.index];
                self.index += 1;
                Ok(Some(element))
            } else {
                Ok(None)
            }
        }
    }

    let elements = vec![IgnoredAny, IgnoredAny];
    let access = TestAccess { elements, index: 0 };
    let result = visit_seq(access);
    assert_eq!(result.unwrap(), IgnoredAny);
}

