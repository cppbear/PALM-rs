// Answer 0

#[test]
fn test_new_function() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyIterator<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> MyIterator<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets = vec![
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
        Bucket { key: 3, value: "three" },
    ];
    
    let iterator = MyIterator::new(buckets);
    let mut iter = iterator.iter;

    assert_eq!(iter.next().unwrap().key, 1);
    assert_eq!(iter.next().unwrap().value, "two");
    assert_eq!(iter.next().unwrap().key, 3);
    assert!(iter.next().is_none());
}

#[test]
fn test_new_function_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyIterator<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> MyIterator<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets: Vec<Bucket<i32, &str>> = Vec::new();
    let iterator = MyIterator::new(buckets);
    let mut iter = iterator.iter;

    assert!(iter.next().is_none());
}

