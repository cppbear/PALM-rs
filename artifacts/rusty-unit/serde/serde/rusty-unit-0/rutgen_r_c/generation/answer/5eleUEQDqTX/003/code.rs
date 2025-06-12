// Answer 0

#[test]
fn test_visit_map_success() {
    struct MockMapAccess {
        entries: Vec<(Content, Content)>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_entry(&mut self) -> Result<Option<((Content, Content))>, Self::Error> {
            if self.index < self.entries.len() {
                let entry = self.entries[self.index].clone();
                self.index += 1;
                Ok(Some(entry))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.entries.len())
        }
    }

    let entries = vec![
        (Content::String("key1".to_string()), Content::U8(1)),
        (Content::String("key2".to_string()), Content::U16(2)),
    ];
    let map_access = MockMapAccess { entries, index: 0 };
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_map(map_access);
    let expected = Content::Map(vec![
        (Content::String("key1".to_string()), Content::U8(1)),
        (Content::String("key2".to_string()), Content::U16(2)),
    ]);
    
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_visit_map_failure() {
    struct MockMapAccess {
        should_fail: bool,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_entry(&mut self) -> Result<Option<((Content, Content))>, Self::Error> {
            if self.should_fail {
                Err(())
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(1)
        }
    }

    let map_access = MockMapAccess { should_fail: true };
    let visitor = ContentVisitor { value: PhantomData };
    let result = visitor.visit_map(map_access);
    
    assert!(result.is_err());
}

