// Answer 0

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IteratorStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries: Vec<Bucket<i32, &str>> = vec![];
    let iterator = IteratorStruct::new(&entries);
    
    assert!(iterator.iter.clone().count() == 0);
}

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IteratorStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    impl<'a, K, V> IteratorStruct<'a, K, V> {
        pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let entries: Vec<Bucket<i32, &str>> = vec![
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
    ];
    let iterator = IteratorStruct::new(&entries);

    let mut count = 0;
    for _ in iterator.iter.clone() {
        count += 1;
    }

    assert_eq!(count, 2);
}

