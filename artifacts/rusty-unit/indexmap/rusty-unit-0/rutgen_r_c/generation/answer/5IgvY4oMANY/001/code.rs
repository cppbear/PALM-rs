// Answer 0

#[test]
fn test_swap_remove_finish_last_index() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "value1" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.swap_remove_finish(0);
    assert_eq!(result, (1, "value1"));
}

#[test]
fn test_swap_remove_finish_non_last_index() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "value1" },
        Bucket { hash: HashValue(2), key: 2, value: "value2" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.swap_remove_finish(0);
    assert_eq!(result, (1, "value1"));
}

#[test]
#[should_panic]
fn test_swap_remove_finish_empty_entries() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, &str>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // This should panic since there are no entries to swap
    let _ = ref_mut.swap_remove_finish(0);
}

#[test]
fn test_swap_remove_finish_multiple_elements() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "value1" },
        Bucket { hash: HashValue(2), key: 2, value: "value2" },
        Bucket { hash: HashValue(3), key: 3, value: "value3" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let result = ref_mut.swap_remove_finish(1);
    assert_eq!(result, (2, "value2"));
}

