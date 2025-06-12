// Answer 0

#[test]
fn test_decrement_indices_with_no_shift() {
    let mut indices = hash_table::HashTable::new();
    let capacity = 4;
    indices.reserve(capacity);
    for i in 0..capacity {
        indices.insert(i, i);
    }
    
    let entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: "a" },
        Bucket { hash: HashValue(1), key: 1, value: "b" },
        Bucket { hash: HashValue(2), key: 2, value: "c" },
        Bucket { hash: HashValue(3), key: 3, value: "d" },
    ];

    let entries_refmut = RefMut::new(&mut indices, &mut entries);

    // scenario where shifted_entries.len() is equal to self.indices.capacity() / 2
    let start = 2;
    let end = 4;
    
    entries_refmut.decrement_indices(start, end);
    
    // Check indices after decrement
    assert_eq!(indices.find(&HashValue(2).0).unwrap(), 1);
    assert_eq!(indices.find(&HashValue(3).0).unwrap(), 2);
}

#[test]
#[should_panic]
fn test_decrement_indices_with_invalid_range() {
    let mut indices = hash_table::HashTable::new();
    let capacity = 4;
    indices.reserve(capacity);
    for i in 0..capacity {
        indices.insert(i, i);
    }
    
    let entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: "a" },
        Bucket { hash: HashValue(1), key: 1, value: "b" },
        Bucket { hash: HashValue(2), key: 2, value: "c" },
        Bucket { hash: HashValue(3), key: 3, value: "d" },
    ];

    let entries_refmut = RefMut::new(&mut indices, &mut entries);

    // This range is invalid as start is not less than end
    let start = 4;
    let end = 2;
    
    entries_refmut.decrement_indices(start, end);
}

#[test]
fn test_decrement_indices_with_full_sweep() {
    let mut indices = hash_table::HashTable::new();
    let capacity = 4;
    indices.reserve(capacity);
    for i in 0..capacity {
        indices.insert(i, i);
    }
    
    let entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: "a" },
        Bucket { hash: HashValue(1), key: 1, value: "b" },
        Bucket { hash: HashValue(2), key: 2, value: "c" },
        Bucket { hash: HashValue(3), key: 3, value: "d" },
    ];

    let entries_refmut = RefMut::new(&mut indices, &mut entries);

    // scenario where shifted_entries.len() is greater than self.indices.capacity() / 2
    let start = 0;
    let end = capacity;

    entries_refmut.decrement_indices(start, end);
    
    // Check indices after decrement
    assert_eq!(indices.find(&HashValue(0).0).unwrap(), 0);
    assert_eq!(indices.find(&HashValue(1).0).unwrap(), 1);
}

#[test]
fn test_decrement_indices_with_no_available_space() {
    let mut indices = hash_table::HashTable::new();
    let capacity = 4;
    indices.reserve(capacity);
    for i in 0..capacity {
        indices.insert(i, i + 1);
    }
    
    let entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: "x" },
        Bucket { hash: HashValue(1), key: 1, value: "y" },
    ];

    let entries_refmut = RefMut::new(&mut indices, &mut entries);

    // This scenario should not panic, but effectively move indices
    let start = 0;
    let end = 2;  // len(entries) is 2, and capacity() / 2 is also 2
    
    entries_refmut.decrement_indices(start, end);
    
    // Check altered indices
    assert_eq!(indices.find(&HashValue(1).0).unwrap(), 0);
    assert_eq!(indices.find(&HashValue(2).0).unwrap(), 1);
}

