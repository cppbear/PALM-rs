// Answer 0

#[test]
fn test_next_pair_some() {
    struct TestIterator {
        items: Vec<(i32, i32)>,
        index: usize,
    }

    impl Iterator for TestIterator {
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

    let items = vec![(1, 2), (3, 4)];
    let mut deserializer = MapDeserializer {
        iter: items.into_iter().fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    assert_eq!(deserializer.next_pair(), Some((1, 2)));
    assert_eq!(deserializer.count, 1);
    assert_eq!(deserializer.next_pair(), Some((3, 4)));
    assert_eq!(deserializer.count, 2);
}

#[test]
fn test_next_pair_none() {
    struct TestIterator {
        items: Vec<(i32, i32)>,
        index: usize,
    }

    impl Iterator for TestIterator {
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

    let items: Vec<(i32, i32)> = vec![];
    let mut deserializer = MapDeserializer {
        iter: items.into_iter().fuse(),
        value: None,
        count: 0,
        lifetime: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };

    assert_eq!(deserializer.next_pair(), None);
    assert_eq!(deserializer.count, 0);
}

