// Answer 0

#[test]
fn test_seq_deserializer_new() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(self.count as i32)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { count: 0 };
    let deserializer: SeqDeserializer<TestIterator, ()> = SeqDeserializer::new(iter);

    assert_eq!(deserializer.count, 0);
}

#[test]
fn test_seq_deserializer_new_with_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator;
    let deserializer: SeqDeserializer<EmptyIterator, ()> = SeqDeserializer::new(iter);

    assert_eq!(deserializer.count, 0);
}

