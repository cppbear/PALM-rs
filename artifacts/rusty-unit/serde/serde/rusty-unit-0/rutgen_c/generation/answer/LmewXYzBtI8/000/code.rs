// Answer 0

#[test]
fn test_size_hint_with_elements() {
    struct TestItem;
    struct TestPair(TestItem, TestItem);
    
    impl private::Pair for TestPair {}
    
    struct TestIterator {
        count: usize,
    }
    
    impl Iterator for TestIterator {
        type Item = TestPair;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(TestPair(TestItem, TestItem))
            } else {
                None
            }
        }
    }
    
    let iter = TestIterator { count: 3 }.fuse();
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 3,
        lifetime: PhantomData,
        error: PhantomData,
    };
    
    assert_eq!(deserializer.size_hint(), Some(3));
}

#[test]
fn test_size_hint_with_no_elements() {
    struct TestPair;

    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = TestPair;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator.fuse();
    let deserializer = MapDeserializer {
        iter,
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    assert_eq!(deserializer.size_hint(), None);
}

#[test]
fn test_size_hint_with_partial_elements() {
    struct TestItem;
    struct TestPair(TestItem, TestItem);
    
    impl private::Pair for TestPair {}
    
    struct TestIterator {
        count: usize,
    }
    
    impl Iterator for TestIterator {
        type Item = TestPair;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(TestPair(TestItem, TestItem))
            } else {
                None
            }
        }
    }
    
    let iter = TestIterator { count: 5 }.fuse();
    let deserializer = MapDeserializer {
        iter,
        value: Some(Second(TestItem)),
        count: 5,
        lifetime: PhantomData,
        error: PhantomData,
    };
    
    assert_eq!(deserializer.size_hint(), Some(5));
}

