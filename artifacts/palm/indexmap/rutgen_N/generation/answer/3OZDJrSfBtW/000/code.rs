// Answer 0

#[test]
fn test_new_with_valid_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IteratorWrapper<K, V> {
        iter: std::slice::IterMut<Bucket<K, V>>,
    }

    impl<K, V> IteratorWrapper<K, V> {
        pub(super) fn new(entries: &mut [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter_mut(),
            }
        }
    }

    let mut buckets = [
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];

    let iter_wrapper = IteratorWrapper::new(&mut buckets);
    let mut iter = iter_wrapper.iter;

    assert_eq!(iter.next().unwrap().key, 1);
    assert_eq!(iter.next().unwrap().key, 2);
    assert!(iter.next().is_none());
}

#[test]
#[should_panic]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IteratorWrapper<K, V> {
        iter: std::slice::IterMut<Bucket<K, V>>,
    }

    impl<K, V> IteratorWrapper<K, V> {
        pub(super) fn new(entries: &mut [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter_mut(),
            }
        }
    }

    let mut buckets: [Bucket<i32, &str>; 0] = []; // Empty bucket array

    let _iter_wrapper = IteratorWrapper::new(&mut buckets); // This is expected to work, but will have no elements to iterate.
}

