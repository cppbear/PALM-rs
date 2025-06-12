// Answer 0

#[test]
fn test_decrement_indices_full_sweep() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    
    // Simulate filling indices
    indices.insert(1, 0);
    indices.insert(2, 1);
    indices.insert(3, 2);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 3);
    
    assert_eq!(*indices.find(1).unwrap(), 0);
    assert_eq!(*indices.find(2).unwrap(), 0);
    assert_eq!(*indices.find(3).unwrap(), 1);
}

#[test]
fn test_decrement_indices_find_each_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    
    // Simulate filling indices
    indices.insert(1, 0);
    indices.insert(2, 1);
    indices.insert(3, 2);
    indices.insert(4, 3);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 3); // Decrement for entries 1 and 2
    
    assert_eq!(*indices.find(1).unwrap(), 0);
    assert_eq!(*indices.find(2).unwrap(), 0);
    assert_eq!(*indices.find(3).unwrap(), 1);
    assert_eq!(*indices.find(4).unwrap(), 3);
}

#[test]
#[should_panic]
fn test_decrement_indices_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![Bucket { hash: HashValue(1), key: 1, value: 10 }];
    
    indices.insert(1, 0);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 10); // This should panic due to out of bounds
}

