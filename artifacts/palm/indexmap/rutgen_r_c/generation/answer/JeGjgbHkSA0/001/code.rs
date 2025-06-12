// Answer 0

#[test]
fn test_into_keys_non_empty() {
    struct LocalBucket<K, V> {
        hash: usize,
        key: K,
        value: V,
    }

    let entries: Vec<LocalBucket<_, _>> = vec![
        LocalBucket { hash: 1, key: "key1", value: "value1" },
        LocalBucket { hash: 2, key: "key2", value: "value2" },
    ];

    let slice = Box::new(Slice { entries });
    let into_keys = slice.into_keys();

    assert_eq!(into_keys.iter.len(), 2);
}

#[test]
fn test_into_keys_empty() {
    let entries: Vec<Bucket<(), ()>> = vec![];
    let slice = Box::new(Slice { entries });
    let into_keys = slice.into_keys();

    assert_eq!(into_keys.iter.len(), 0);
}

#[test]
fn test_into_keys_single_entry() {
    struct LocalBucket<K, V> {
        hash: usize,
        key: K,
        value: V,
    }

    let entries: Vec<LocalBucket<_, _>> = vec![
        LocalBucket { hash: 1, key: "single_key", value: "single_value" },
    ];

    let slice = Box::new(Slice { entries });
    let into_keys = slice.into_keys();

    assert_eq!(into_keys.iter.len(), 1);
}

