// Answer 0

#[test]
fn test_move_index_valid_case() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![Bucket { hash: HashValue(1), key: 0, value: "A" }, 
                            Bucket { hash: HashValue(2), key: 1, value: "B" }, 
                            Bucket { hash: HashValue(3), key: 2, value: "C" }];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(0, 2);
    
    assert_eq!(entries[0], Bucket { hash: HashValue(2), key: 1, value: "B" });
    assert_eq!(entries[1], Bucket { hash: HashValue(1), key: 0, value: "A" });
    assert_eq!(entries[2], Bucket { hash: HashValue(3), key: 2, value: "C" });
}

#[test]
#[should_panic]
fn test_move_index_invalid_from_index() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, &str>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: "A" },
        Bucket { hash: HashValue(2), key: 1, value: "B" },
        Bucket { hash: HashValue(3), key: 2, value: "C" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(5, 2); // from index out of bounds
}

#[test]
#[should_panic]
fn test_move_index_invalid_to_index() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, &str>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: "A" },
        Bucket { hash: HashValue(2), key: 1, value: "B" },
        Bucket { hash: HashValue(3), key: 2, value: "C" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(1, 5); // to index out of bounds
}

#[test]
#[should_panic]
fn test_move_index_same_indices() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, &str>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: "A" },
        Bucket { hash: HashValue(2), key: 1, value: "B" },
        Bucket { hash: HashValue(3), key: 2, value: "C" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(1, 1); // from and to indices are the same
}

#[test]
#[should_panic]
fn test_move_index_invalid_range() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, &str>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: "A" },
        Bucket { hash: HashValue(2), key: 1, value: "B" },
        Bucket { hash: HashValue(3), key: 2, value: "C" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(2, 0); // trying to move from a higher index to a lower one, which is invalid
}

