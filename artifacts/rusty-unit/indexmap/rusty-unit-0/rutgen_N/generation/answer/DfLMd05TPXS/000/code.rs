// Answer 0

#[test]
fn test_get_hash_with_valid_index() {
    struct Bucket<K, V> {
        hash: HashWrapper,
        key: K,
        value: V,
    }

    struct HashWrapper {
        value: u64,
    }

    impl HashWrapper {
        fn get(&self) -> u64 {
            self.value
        }
    }

    let entries = vec![
        Bucket { hash: HashWrapper { value: 10 }, key: "key1", value: "value1" },
        Bucket { hash: HashWrapper { value: 20 }, key: "key2", value: "value2" },
        Bucket { hash: HashWrapper { value: 30 }, key: "key3", value: "value3" },
    ];

    let hash_fn = get_hash(&entries);
    assert_eq!(hash_fn(&0), 10);
    assert_eq!(hash_fn(&1), 20);
    assert_eq!(hash_fn(&2), 30);
}

#[test]
#[should_panic]
fn test_get_hash_with_invalid_index() {
    struct Bucket<K, V> {
        hash: HashWrapper,
        key: K,
        value: V,
    }

    struct HashWrapper {
        value: u64,
    }

    impl HashWrapper {
        fn get(&self) -> u64 {
            self.value
        }
    }

    let entries = vec![
        Bucket { hash: HashWrapper { value: 10 }, key: "key1", value: "value1" },
    ];

    let hash_fn = get_hash(&entries);
    // This access should panic due to out-of-bounds index
    let _ = hash_fn(&1);
}

