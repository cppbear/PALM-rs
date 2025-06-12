// Answer 0

#[test]
fn test_move_index_same_position() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 1); // Test moving index to the same position
    assert_eq!(entries.len(), 3); // Ensure no entries have been added/removed
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 3); // This will panic due to out-of-bounds access
}

#[test]
fn test_move_index_forward() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 2); // Move from index 0 to index 2
    assert_eq!(entries[0].key, "key2"); // Verify that the content has moved accordingly
    assert_eq!(entries[1].key, "key3"); // Verify order
    assert_eq!(entries[2].key, "key1"); // The moved element should now be at the end
}

#[test]
fn test_move_index_backward() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 0); // Move from index 2 to index 0
    assert_eq!(entries[0].key, "key3"); // Verify that the content has moved accordingly
    assert_eq!(entries[1].key, "key1"); // Verify order
    assert_eq!(entries[2].key, "key2"); // The moved element should now be at the end
}

