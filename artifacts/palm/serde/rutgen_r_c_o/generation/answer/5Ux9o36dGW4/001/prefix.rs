// Answer 0

#[test]
fn test_next_pair_valid_input() {
    struct TestIterator {
        data: Vec<(i32, String)>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = (i32, String);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let data = vec![(1, "one".to_string()), (2, "two".to_string())];
    let mut iter = TestIterator { data, index: 0 };
    let mut deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result = deserializer.next_pair();
}

#[test]
fn test_next_pair_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = (i32, String);

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator;
    let mut deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result = deserializer.next_pair();
}

#[test]
fn test_next_pair_single_element() {
    struct SingleElementIterator {
        called: bool,
    }

    impl Iterator for SingleElementIterator {
        type Item = (i32, String);

        fn next(&mut self) -> Option<Self::Item> {
            if !self.called {
                self.called = true;
                Some((3, "three".to_string()))
            } else {
                None
            }
        }
    }

    let iter = SingleElementIterator { called: false };
    let mut deserializer = MapDeserializer {
        iter: iter.fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    let result = deserializer.next_pair();
}

