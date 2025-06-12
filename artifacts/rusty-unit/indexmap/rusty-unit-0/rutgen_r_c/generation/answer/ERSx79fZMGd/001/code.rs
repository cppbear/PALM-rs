// Answer 0

fn test_insert_bulk_no_grow_success() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.try_reserve(2).unwrap(); // Ensure enough capacity
    let entries = [
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

fn test_insert_bulk_no_grow_panic() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.try_reserve(1).unwrap(); // Only enough capacity for 1 element
    let entries = [
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ];
    let result = std::panic::catch_unwind(|| {
        insert_bulk_no_grow(&mut indices, &entries);
    });
    assert!(result.is_err());
}

fn test_insert_bulk_no_grow_empty_entries() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.try_reserve(0).unwrap(); // Sufficient for no entries
    let entries: Vec<Bucket<_, _>> = Vec::new(); // No entries
    insert_bulk_no_grow(&mut indices, &entries);
}

fn test_insert_bulk_no_grow_exceed_capacity() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.try_reserve(2).unwrap(); // Capacity for 2
    let entries = [
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
        Bucket { hash: HashValue(3), key: 30, value: "c" },
    ];
    let result = std::panic::catch_unwind(|| {
        insert_bulk_no_grow(&mut indices, &entries);
    });
    assert!(result.is_err());
}

