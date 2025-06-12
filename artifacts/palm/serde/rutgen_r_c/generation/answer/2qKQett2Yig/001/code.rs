// Answer 0

#[test]
fn test_end_with_remaining_elements() {
    struct DummyError;

    impl de::Error for DummyError {
        fn custom(_msg: &str) -> Self {
            DummyError
        }
    }

    struct TestIterator {
        current: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                let item = self.current as i32;
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { current: 0, limit: 3 };
    let count = 2; // we expect 2 elements but the iterator has 3

    let seq_deserializer = SeqDeserializer {
        iter: iter.fuse(),
        count,
        marker: PhantomData::<DummyError>,
    };

    let result = seq_deserializer.end();
    assert!(result.is_err());
} 

#[test]
fn test_end_with_no_remaining_elements() {
    struct DummyError;

    impl de::Error for DummyError {
        fn custom(_msg: &str) -> Self {
            DummyError
        }
    }

    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator;
    let count = 0; // we expect 0 elements and also there are no remaining elements

    let seq_deserializer = SeqDeserializer {
        iter: iter.fuse(),
        count,
        marker: PhantomData::<DummyError>,
    };

    let result = seq_deserializer.end();
    assert!(result.is_ok());
} 

#[test]
fn test_end_with_remaining_elements_equal_to_expected() {
    struct DummyError;

    impl de::Error for DummyError {
        fn custom(_msg: &str) -> Self {
            DummyError
        }
    }

    struct TestIterator {
        current: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                let item = self.current as i32;
                self.current += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let iter = TestIterator { current: 0, limit: 2 };
    let count = 2; // we have 2 expected, iterator also has 2

    let seq_deserializer = SeqDeserializer {
        iter: iter.fuse(),
        count,
        marker: PhantomData::<DummyError>,
    };

    let result = seq_deserializer.end();
    assert!(result.is_err());
} 

