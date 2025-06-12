// Answer 0

#[test]
fn test_decrement_indices_no_shift_due_to_capacity_half() {
    let mut indices = hashbrown::HashMap::new();
    let capacity = 4;
    for i in 0..capacity {
        indices.insert(i, i);
    }
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: "one" },
        Bucket { hash: HashValue(1), key: 2, value: "two" },
        Bucket { hash: HashValue(2), key: 3, value: "three" },
        Bucket { hash: HashValue(3), key: 4, value: "four" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(2, 4); // shifted_entries.len() == self.indices.capacity() / 2

    assert_eq!(indices.get(&0), Some(&0)); // no changes in indices for 0
    assert_eq!(indices.get(&1), Some(&1)); // no changes in indices for 1
    assert_eq!(indices.get(&2), Some(&2)); // no changes in indices for 2
    assert_eq!(indices.get(&3), Some(&3)); // no changes in indices for 3
}

#[test]
#[should_panic]
fn test_decrement_indices_panics_on_empty_range() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, &str>> = vec![];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0); // accessing an empty range should panic
}

#[test]
fn test_decrement_indices_no_shift_due_to_empty_entries() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, &str>> = vec![];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0); // no entries to shift
    assert!(indices.is_empty());
}

#[test]
fn test_decrement_indices_no_shift_due_to_invalid_range() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..4 {
        indices.insert(i, i);
    }
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 1, value: "one" },
        Bucket { hash: HashValue(1), key: 2, value: "two" },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(2, 2); // start and end are equal, should not cause any shifts

    assert_eq!(indices.get(&0), Some(&0)); // no changes in indices for 0
    assert_eq!(indices.get(&1), Some(&1)); // no changes in indices for 1
    assert_eq!(indices.get(&2), Some(&2)); // no changes in indices for 2
    assert_eq!(indices.get(&3), Some(&3)); // no changes in indices for 3
}

