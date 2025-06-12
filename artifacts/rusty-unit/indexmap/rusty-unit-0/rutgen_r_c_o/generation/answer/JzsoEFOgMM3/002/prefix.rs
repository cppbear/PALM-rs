// Answer 0

#[test]
fn test_decrement_indices_with_empty_range() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.decrement_indices(0, 0);
}

#[test]
fn test_decrement_indices_with_single_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![Bucket { hash: HashValue(0), key: 1, value: 2 }];
    for i in 0..1 {
        indices.insert(i, i);
    }
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.decrement_indices(0, 1);
}

#[test]
fn test_decrement_indices_with_multiple_entries() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 2 },
        Bucket { hash: HashValue(1), key: 3, value: 4 },
        Bucket { hash: HashValue(2), key: 5, value: 6 }
    ];
    
    for i in 0..3 {
        indices.insert(i, i);
    }
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.decrement_indices(0, 3);
}

#[test]
fn test_decrement_indices_with_capacity() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 2 },
        Bucket { hash: HashValue(1), key: 3, value: 4 },
        Bucket { hash: HashValue(2), key: 5, value: 6 },
        Bucket { hash: HashValue(3), key: 7, value: 8 },
        Bucket { hash: HashValue(4), key: 9, value: 10 },
    ];
    
    for i in 0..5 {
        indices.insert(i, i);
    }

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.decrement_indices(0, 5);
}

#[test]
fn test_decrement_indices_with_full_capacity() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = (0..6).map(|i| Bucket { hash: HashValue(i), key: i, value: i * 2 }).collect();
    
    for i in 0..6 {
        indices.insert(i, i);
    }

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.decrement_indices(0, 6);
}

