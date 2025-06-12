// Answer 0

#[test]
fn test_swap_indices_equal_indices() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 0); // should not panic and do nothing
}

#[test]
fn test_swap_indices_valid_swap() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    // Simulating the indices for the test
    indices.insert(1 as u64, 0);
    indices.insert(2 as u64, 1);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1); // should not panic and swap the entries
    assert_eq!(entries[0].key, 2);
    assert_eq!(entries[1].key, 1);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_invalid_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    // Simulating only one index in the hash table. 
    indices.insert(1 as u64, 0);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1); // this should panic
}

