// Answer 0

#[test]
fn test_shift_remove_finish() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(1);

    assert_eq!(key, 2);
    assert_eq!(value, 20);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 3);
}

#[test]
#[should_panic]
fn test_shift_remove_finish_out_of_bounds() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let _ = ref_mut.shift_remove_finish(2); // Out of bounds
}

#[test]
fn test_shift_remove_finish_last_element() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let (key, value) = ref_mut.shift_remove_finish(1);

    assert_eq!(key, 2);
    assert_eq!(value, 20);
    assert!(entries.is_empty());
}

