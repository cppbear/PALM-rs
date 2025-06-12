// Answer 0

#[test]
fn test_as_slice_empty_iterator() {
    struct TestIterator {
        items: Vec<(i32, i32)>,
        position: usize,
    }

    impl TestIterator {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                position: 0,
            }
        }

        fn as_slice(&self) -> &[i32] {
            &self.items[self.position..]
        }
    }

    struct Slice<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> Slice<K, V> {
        fn from_slice(slice: &[(K, V)]) -> &Self {
            unsafe { &*(slice as *const _ as *const Self) }
        }
    }

    let iter = TestIterator::new();
    let slice: &Slice<i32, i32> = Slice::from_slice(iter.as_slice());

    assert_eq!(slice.data.len(), 0);
}

#[test]
fn test_as_slice_non_empty_iterator() {
    struct TestIterator {
        items: Vec<(i32, i32)>,
        position: usize,
    }

    impl TestIterator {
        fn new(items: Vec<(i32, i32)>) -> Self {
            Self {
                items,
                position: 0,
            }
        }

        fn as_slice(&self) -> &[(i32, i32)] {
            &self.items[self.position..]
        }
    }

    struct Slice<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> Slice<K, V> {
        fn from_slice(slice: &[(K, V)]) -> &Self {
            unsafe { &*(slice as *const _ as *const Self) }
        }
    }

    let iter = TestIterator::new(vec![(1, 2), (3, 4)]);
    let slice: &Slice<i32, i32> = Slice::from_slice(iter.as_slice());

    assert_eq!(slice.data.len(), 2);
    assert_eq!(slice.data[0], (1, 2));
    assert_eq!(slice.data[1], (3, 4));
}

