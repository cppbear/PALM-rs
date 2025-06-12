// Answer 0

#[test]
fn test_increment_indices_shift_all_indices() {
    let mut indices = vec![0, 1, 2, 3, 4].into_iter().collect::<hashbrown::HashMap<_, _>>();
    let hash_values = vec![HashValue(0), HashValue(1), HashValue(2), HashValue(3)];
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_values[0], key: 0, value: 0 },
        Bucket { hash: hash_values[1], key: 1, value: 1 },
        Bucket { hash: hash_values[2], key: 2, value: 2 },
        Bucket { hash: hash_values[3], key: 3, value: 3 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.increment_indices(0, 2);

    assert_eq!(indices.get(&0), None); // Index 0 should be removed
    assert_eq!(indices.get(&1), Some(&1)); // Index 1 should remain
    assert_eq!(indices.get(&2), Some(&2)); // Index 2 remains
    assert_eq!(indices.get(&3), Some(&3)); // Index 3 remains
    assert_eq!(indices.get(&4), Some(&4)); // Index 4 remains
}

#[test]
#[should_panic]
fn test_increment_indices_panic_out_of_bounds() {
    let mut indices = vec![0, 1, 2].into_iter().collect::<hashbrown::HashMap<_, _>>();
    let hash_values = vec![HashValue(0), HashValue(1), HashValue(2)];
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_values[0], key: 0, value: 0 },
        Bucket { hash: hash_values[1], key: 1, value: 1 },
        Bucket { hash: hash_values[2], key: 2, value: 2 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.increment_indices(1, 4); // Should panic due to out-of-bounds
}

#[test]
fn test_increment_indices_not_exceed_half_capacity() {
    let mut indices = vec![0, 1, 3].into_iter().collect::<hashbrown::HashMap<_, _>>();
    let mut entries = vec![Bucket { hash: HashValue(0), key: 0, value: 0 }, 
                           Bucket { hash: HashValue(1), key: 1, value: 1 },
                           Bucket { hash: HashValue(2), key: 2, value: 2 },
                           Bucket { hash: HashValue(3), key: 3, value: 3 }];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.increment_indices(1, 3);

    assert_eq!(indices.get(&1), None); // Index 1 should be removed
    assert_eq!(indices.get(&2), Some(&2)); // Index 2 shifted to 2
    assert_eq!(indices.get(&3), Some(&3)); // Index 3 should remain
}

#[test]
fn test_increment_indices_with_empty_entries() {
    let mut indices = vec![0, 1, 2].into_iter().collect::<hashbrown::HashMap<_, _>>();
    let entries: Vec<Bucket<usize, usize>> = vec![];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.increment_indices(0, 0); // No operation should happen

    assert_eq!(indices.get(&0), Some(&0)); // Check that index 0 is still present
}

