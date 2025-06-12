// Answer 0

#[test]
fn test_next_pair_with_elements() {
    struct MockIterator {
        items: Vec<(i32, i32)>,
        index: usize,
    }

    impl Iterator for MockIterator {
        type Item = (i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct TestStruct<I> {
        iter: I,
        count: usize,
    }

    impl<I> TestStruct<I>
    where
        I: Iterator<Item = (i32, i32)>,
    {
        fn next_pair(&mut self) -> Option<(i32, i32)> {
            match self.iter.next() {
                Some(kv) => {
                    self.count += 1;
                    Some(kv)
                }
                None => None,
            }
        }
    }

    let items = vec![(1, 2), (3, 4)];
    let mut mock_iter = MockIterator { items, index: 0 };
    let mut test_struct = TestStruct { iter: mock_iter, count: 0 };

    assert_eq!(test_struct.next_pair(), Some((1, 2)));
    assert_eq!(test_struct.count, 1);
    assert_eq!(test_struct.next_pair(), Some((3, 4)));
    assert_eq!(test_struct.count, 2);
    assert_eq!(test_struct.next_pair(), None);
    assert_eq!(test_struct.count, 2);
}

#[test]
fn test_next_pair_empty_iterator() {
    struct MockIterator {
        items: Vec<(i32, i32)>,
        index: usize,
    }

    impl Iterator for MockIterator {
        type Item = (i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.items.len() {
                let item = self.items[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    struct TestStruct<I> {
        iter: I,
        count: usize,
    }

    impl<I> TestStruct<I>
    where
        I: Iterator<Item = (i32, i32)>,
    {
        fn next_pair(&mut self) -> Option<(i32, i32)> {
            match self.iter.next() {
                Some(kv) => {
                    self.count += 1;
                    Some(kv)
                }
                None => None,
            }
        }
    }

    let items: Vec<(i32, i32)> = vec![];
    let mut mock_iter = MockIterator { items, index: 0 };
    let mut test_struct = TestStruct { iter: mock_iter, count: 0 };

    assert_eq!(test_struct.next_pair(), None);
    assert_eq!(test_struct.count, 0);
}

