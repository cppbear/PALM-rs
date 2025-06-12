// Answer 0

#[test]
fn test_seq_deserializer_new_empty_iterator() {
    struct TestIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0, limit: 0 };
    let seq_deserializer: SeqDeserializer<_, ()> = SeqDeserializer::new(iter);
    assert_eq!(seq_deserializer.count, 0);
}

#[test]
fn test_seq_deserializer_new_filled_iterator() {
    struct TestIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0, limit: 5 };
    let seq_deserializer: SeqDeserializer<_, ()> = SeqDeserializer::new(iter);
    assert_eq!(seq_deserializer.count, 0);
}

#[test]
fn test_seq_deserializer_new_large_iterator() {
    struct TestIterator {
        count: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.limit {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0, limit: 1000 };
    let seq_deserializer: SeqDeserializer<_, ()> = SeqDeserializer::new(iter);
    assert_eq!(seq_deserializer.count, 0);
}

