// Answer 0

#[test]
fn test_get_hash_valid_index() {
    struct Bucket<K, V> {
        hash: std::cell::Cell<u64>,
        key: K,
        value: V,
    }

    let entries = vec![
        Bucket {
            hash: std::cell::Cell::new(10),
            key: "key1",
            value: "value1",
        },
        Bucket {
            hash: std::cell::Cell::new(20),
            key: "key2",
            value: "value2",
        },
    ];

    let hash_function = get_hash(&entries);
    assert_eq!(hash_function(&0), 10);
    assert_eq!(hash_function(&1), 20);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_get_hash_invalid_index() {
    struct Bucket<K, V> {
        hash: std::cell::Cell<u64>,
        key: K,
        value: V,
    }

    let entries = vec![
        Bucket {
            hash: std::cell::Cell::new(30),
            key: "key1",
            value: "value1",
        },
    ];

    let hash_function = get_hash(&entries);
    // This should panic since the valid index is 0
    let _ = hash_function(&1);
}

#[test]
fn test_get_hash_empty_entries() {
    struct Bucket<K, V> {
        hash: std::cell::Cell<u64>,
        key: K,
        value: V,
    }

    let entries: Vec<Bucket<&str, &str>> = vec![];

    let hash_function = get_hash(&entries);
    // Should not panic, but also will not provide a valid output
    // Attempting to access the hash with valid index returns no result
    assert!(std::panic::catch_unwind(|| { hash_function(&0); }).is_err());
}

