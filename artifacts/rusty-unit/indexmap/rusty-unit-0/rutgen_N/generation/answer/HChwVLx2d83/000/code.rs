// Answer 0

#[test]
fn test_as_slice_empty_iterator() {
    struct TestIterator<K, V> {
        iter: Vec<(K, V)>,
    }

    impl<K, V> TestIterator<K, V> {
        pub fn as_slice(&self) -> &[K] {
            self.iter.iter().map(|(k, _)| k).collect::<Vec<&K>>().as_slice()
        }
    }

    let iter: TestIterator<u32, String> = TestIterator { iter: vec![] };
    let slice = iter.as_slice();
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_as_slice_one_entry() {
    struct TestIterator<K, V> {
        iter: Vec<(K, V)>,
    }

    impl<K, V> TestIterator<K, V> {
        pub fn as_slice(&self) -> &[K] {
            self.iter.iter().map(|(k, _)| k).collect::<Vec<&K>>().as_slice()
        }
    }

    let iter = TestIterator {
        iter: vec![(1, "One".to_string())],
    };
    let slice = iter.as_slice();
    assert_eq!(slice.len(), 1);
    assert_eq!(*slice[0], 1);
}

#[test]
fn test_as_slice_multiple_entries() {
    struct TestIterator<K, V> {
        iter: Vec<(K, V)>,
    }

    impl<K, V> TestIterator<K, V> {
        pub fn as_slice(&self) -> &[K] {
            self.iter.iter().map(|(k, _)| k).collect::<Vec<&K>>().as_slice()
        }
    }

    let iter = TestIterator {
        iter: vec![(1, "One".to_string()), (2, "Two".to_string()), (3, "Three".to_string())],
    };
    let slice = iter.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(*slice[0], 1);
    assert_eq!(*slice[1], 2);
    assert_eq!(*slice[2], 3);
}

