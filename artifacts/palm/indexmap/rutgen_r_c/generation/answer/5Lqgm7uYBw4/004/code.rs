// Answer 0

#[test]
fn test_swap_indices_equal_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(1, 1); // a == b and a < entries.len(), should not panic
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_invalid_index_a() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(2, 1); // a is out of bounds, should panic
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_invalid_index_b() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(1, 2); // b is out of bounds, should panic
}

#[test]
fn test_swap_indices_valid_case() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1); // valid swap, indices should exist, no panic
    assert_eq!(entries[0].key, 2); // verify swap occurred
    assert_eq!(entries[1].key, 1); // verify swap occurred
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_not_found() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(0), key: 2, value: 20 }, // same hash, but different keys
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1); // should panic due to indices not found
}

