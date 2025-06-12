// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Iter<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    impl<'a, K, V> Iter<'a, K, V> {
        pub(super) fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter_mut(),
            }
        }
    }

    let mut entries = [
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    
    let iter = Iter::new(&mut entries);
    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Iter<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    impl<'a, K, V> Iter<'a, K, V> {
        pub(super) fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter_mut(),
            }
        }
    }

    let mut entries: Vec<Bucket<i32, &str>> = Vec::new();
    
    let iter = Iter::new(&mut entries);
    assert_eq!(iter.iter.len(), 0);
}

