// Answer 0

#[test]
fn test_swap_remove_finish_with_last_element() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, String> = vec![
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_finish(0);
    
    assert_eq!(result, (1, "one".to_string()));
    assert!(entries.is_empty());
}

#[test]
fn test_swap_remove_finish_with_multiple_elements() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, String> = vec![
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "two".to_string() },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_finish(0);
    
    assert_eq!(result, (1, "one".to_string()));
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], Bucket { hash: HashValue(2), key: 2, value: "two".to_string() });
}

#[test]
fn test_swap_remove_finish_updates_index() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, String> = vec![
        Bucket { hash: HashValue(1), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(2), key: 2, value: "two".to_string() },
    ];

    // Simulating the situation where the index points to the first entry
    indices.insert(1, 0);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let _ = ref_mut.swap_remove_finish(0);
    
    // Check if the index has been updated correctly
    assert_eq!(indices.get(&2), Some(&0));
}

