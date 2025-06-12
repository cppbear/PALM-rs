// Answer 0

#[test]
fn test_size_hint_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let empty_iter = EmptyIterator;
    let map_deserializer = MapDeserializer {
        iter: empty_iter.fuse(),
        value: None,
        count: 0,
        lifetime: PhantomData,
        error: PhantomData,
    };

    assert_eq!(map_deserializer.size_hint(), Some(0));
}

#[test]
fn test_size_hint_single_element_iterator() {
    struct SingleElementIterator {
        has_value: bool,
    }

    impl Iterator for SingleElementIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            if self.has_value {
                self.has_value = false;
                Some(())
            } else {
                None
            }
        }
    }

    let single_iter = SingleElementIterator { has_value: true };
    let map_deserializer = MapDeserializer {
        iter: single_iter.fuse(),
        value: None,
        count: 1,
        lifetime: PhantomData,
        error: PhantomData,
    };

    assert_eq!(map_deserializer.size_hint(), Some(1));
}

#[test]
fn test_size_hint_multiple_element_iterator() {
    struct MultipleElementIterator {
        count: usize,
        current: usize,
    }

    impl Iterator for MultipleElementIterator {
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

    let multi_iter = MultipleElementIterator {
        count: 5,
        current: 0,
    };
    let map_deserializer = MapDeserializer {
        iter: multi_iter.fuse(),
        value: None,
        count: 5,
        lifetime: PhantomData,
        error: PhantomData,
    };

    assert_eq!(map_deserializer.size_hint(), Some(5));
}

