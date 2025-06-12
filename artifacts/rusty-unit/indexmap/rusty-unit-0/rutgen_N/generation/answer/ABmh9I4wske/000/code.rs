// Answer 0

#[test]
fn test_new() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Iter<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> Iter<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let entries = vec![
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
    ];

    let iter = Iter::new(entries);
    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_new_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct Iter<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> Iter<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let entries: Vec<Bucket<i32, &str>> = vec![];
    let iter = Iter::new(entries);
    assert_eq!(iter.iter.len(), 0);
}

