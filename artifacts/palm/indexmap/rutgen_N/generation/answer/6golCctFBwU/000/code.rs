// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyIterator<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    impl<'a, K, V> MyIterator<'a, K, V> {
        pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let buckets = [
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];

    let my_iterator = MyIterator::new(&buckets);
    assert_eq!(my_iterator.iter.len(), buckets.len());
}

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyIterator<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    impl<'a, K, V> MyIterator<'a, K, V> {
        pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let buckets: Vec<Bucket<i32, &str>> = Vec::new();

    let my_iterator = MyIterator::new(&buckets);
    assert_eq!(my_iterator.iter.len(), buckets.len());
}

