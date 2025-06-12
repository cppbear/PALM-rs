// Answer 0

#[test]
fn test_as_slice_empty() {
    struct TestIterator {
        iter: Vec<(i32, String)>,
        index: usize,
    }

    impl TestIterator {
        fn new() -> Self {
            Self {
                iter: Vec::new(),
                index: 0,
            }
        }

        fn as_slice(&self) -> &[i32] {
            &self.iter[..]
        }
    }

    struct Slice<'a, K, V> {
        data: &'a [(K, V)],
    }

    impl<'a, K, V> Slice<'a, K, V> {
        fn from_slice(data: &'a [(K, V)]) -> &Self {
            &Slice { data }
        }
    }

    let iterator = TestIterator::new();
    let slice = Slice::from_slice(iterator.as_slice());
    assert_eq!(slice.data.len(), 0);
}

#[test]
fn test_as_slice_non_empty() {
    struct TestIterator {
        iter: Vec<(i32, String)>,
        index: usize,
    }

    impl TestIterator {
        fn new(entries: Vec<(i32, String)>) -> Self {
            Self {
                iter: entries,
                index: 0,
            }
        }

        fn as_slice(&self) -> &[(i32, String)] {
            &self.iter[..]
        }
    }

    struct Slice<'a, K, V> {
        data: &'a [(K, V)],
    }

    impl<'a, K, V> Slice<'a, K, V> {
        fn from_slice(data: &'a [(K, V)]) -> &Self {
            &Slice { data }
        }
    }

    let entries = vec![(1, "one".to_string()), (2, "two".to_string())];
    let iterator = TestIterator::new(entries);
    let slice = Slice::from_slice(iterator.as_slice());
    assert_eq!(slice.data.len(), 2);
    assert_eq!(slice.data[0], (1, "one".to_string()));
    assert_eq!(slice.data[1], (2, "two".to_string()));
}

