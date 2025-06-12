// Answer 0

#[test]
fn test_seq_deserializer_end_with_no_remaining_elements() {
    struct MockIterator {
        count: usize,
        current: usize,
    }
    
    impl Iterator for MockIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                self.current += 1;
                Some(())
            } else {
                None
            }
        }
    }

    let iter = MockIterator { count: 0, current: 0 };
    let deserializer = SeqDeserializer { iter: iter.fuse(), count: 0, marker: PhantomData };

    let result = deserializer.end();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_seq_deserializer_end_with_remaining_elements() {
    struct MockIterator {
        count: usize,
        current: usize,
    }
    
    impl Iterator for MockIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                self.current += 1;
                Some(())
            } else {
                None
            }
        }
    }

    let iter = MockIterator { count: 3, current: 0 };
    let deserializer = SeqDeserializer { iter: iter.fuse(), count: 2, marker: PhantomData };

    let result = deserializer.end();
    assert!(result.is_err());
}

#[test]
fn test_seq_deserializer_end_with_exact_elements() {
    struct MockIterator {
        count: usize,
        current: usize,
    }

    impl Iterator for MockIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.count {
                self.current += 1;
                Some(())
            } else {
                None
            }
        }
    }

    let iter = MockIterator { count: 2, current: 0 };
    let deserializer = SeqDeserializer { iter: iter.fuse(), count: 2, marker: PhantomData };

    let result = deserializer.end();
    assert_eq!(result, Ok(()));
}

