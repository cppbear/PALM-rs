// Answer 0

#[test]
fn test_get_hash_valid_index() {
    struct HashValue(u64);
    
    impl HashValue {
        fn get(self) -> u64 {
            self.0
        }
    }
    
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1usize, value: "a" },
        Bucket { hash: HashValue(2), key: 2usize, value: "b" },
    ];
    let hash_fn = get_hash(&entries);
    
    assert_eq!(hash_fn(&0), 1);
    assert_eq!(hash_fn(&1), 2);
}

#[test]
#[should_panic]
fn test_get_hash_out_of_bounds() {
    struct HashValue(u64);
    
    impl HashValue {
        fn get(self) -> u64 {
            self.0
        }
    }
    
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1usize, value: "a" },
    ];
    let hash_fn = get_hash(&entries);
    
    // Accessing an out-of-bounds index should trigger a panic
    let _ = hash_fn(&1);
}

#[test]
fn test_get_hash_empty_entries() {
    struct HashValue(u64);
    
    impl HashValue {
        fn get(self) -> u64 {
            self.0
        }
    }
    
    let entries: Vec<Bucket<usize, &str>> = vec![];
    let hash_fn = get_hash(&entries);
    
    // Running the hash function on an empty entries list should not panic
    // but we cannot access any index, verify by not calling it.
}

#[test]
fn test_get_hash_single_entry() {
    struct HashValue(u64);
    
    impl HashValue {
        fn get(self) -> u64 {
            self.0
        }
    }
    
    let entries = vec![
        Bucket { hash: HashValue(42), key: 1usize, value: "single" },
    ];
    let hash_fn = get_hash(&entries);
    
    assert_eq!(hash_fn(&0), 42);
}

