// Answer 0

#[test]
fn test_next_pair_some() {
    struct MockIter {
        count: usize,
        items: Vec<(i32, String)>,
    }

    impl Iterator for MockIter {
        type Item = (i32, String);

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.items.len() {
                let item = self.items[self.count];
                self.count += 1;
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

    impl<I: Iterator<Item=(i32, String)>> TestStruct<I> {
        fn next_pair(&mut self) -> Option<(i32, String)> {
            match self.iter.next() {
                Some(kv) => {
                    self.count += 1;
                    Some(kv)  // Directly returning kv to simulate Pair::split
                }
                None => None,
            }
        }
    }
    
    let items = vec![(1, String::from("one")), (2, String::from("two"))];
    let mut mock_iter = MockIter { count: 0, items };
    let mut test_struct = TestStruct { iter: mock_iter, count: 0 };

    assert_eq!(test_struct.next_pair(), Some((1, String::from("one"))));
    assert_eq!(test_struct.count, 1);
    
    assert_eq!(test_struct.next_pair(), Some((2, String::from("two"))));
    assert_eq!(test_struct.count, 2);
    
    assert_eq!(test_struct.next_pair(), None);
    assert_eq!(test_struct.count, 2);
}

