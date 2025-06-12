// Answer 0

#[test]
fn test_increment_indices_shift_all_indices() {
    let mut indices = hash_table::HashTable::<usize>::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
        Bucket { hash: HashValue(2), key: 1, value: 20 },
        Bucket { hash: HashValue(3), key: 2, value: 30 },
        Bucket { hash: HashValue(4), key: 3, value: 40 },
    ];

    // Fill indices beyond half the capacity
    for i in 0..4 {
        indices.insert(i);
    }

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 3);

    assert_eq!(indices.get(&2), None); // Indices should have shifted
    assert_eq!(indices.get(&3), Some(&1)); // Entry 1 should now be at index 2
    assert_eq!(indices.get(&4), Some(&2)); // Entry 2 should now be at index 3
}

#[test]
#[should_panic]
fn test_increment_indices_panic_on_range_bounds() {
    let mut indices = hash_table::HashTable::<usize>::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
        Bucket { hash: HashValue(2), key: 1, value: 20 },
        Bucket { hash: HashValue(3), key: 2, value: 30 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 5); // This will panic due to out-of-bounds
}

#[test]
fn test_increment_indices_no_shift_condition() {
    let mut indices = hash_table::HashTable::<usize>::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
        Bucket { hash: HashValue(2), key: 1, value: 20 },
        Bucket { hash: HashValue(3), key: 2, value: 30 },
    ];

    // Fill indices below half the capacity
    for i in 0..2 {
        indices.insert(i);
    }

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 2);

    assert_eq!(indices.get(&0), Some(&1)); // Check that the first entry is shifted
    assert_eq!(indices.get(&1), Some(&0)); // Check that the second entry is shifted
}

