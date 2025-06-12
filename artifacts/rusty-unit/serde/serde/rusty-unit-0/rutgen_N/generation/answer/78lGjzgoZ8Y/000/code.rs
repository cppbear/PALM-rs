// Answer 0

#[test]
fn test_size_hint_some() {
    struct TestIterator {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIterator {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    impl TestIterator {
        fn size_hint(&self) -> Option<usize> {
            Some(self.data.len() - self.index)
        }
    }

    let iter = TestIterator::new(vec![1, 2, 3]);
    assert_eq!(iter.size_hint(), Some(3));
}

#[test]
fn test_size_hint_none() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    impl EmptyIterator {
        fn size_hint(&self) -> Option<usize> {
            Some(0)
        }
    }

    let iter = EmptyIterator;
    assert_eq!(iter.size_hint(), Some(0));
}

