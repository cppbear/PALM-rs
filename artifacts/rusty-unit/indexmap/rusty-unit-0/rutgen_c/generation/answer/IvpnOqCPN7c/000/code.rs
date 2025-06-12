// Answer 0

#[test]
fn test_move_index_same_index() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.reserve(5);
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 1);
    assert_eq!(entries[1].key, 2);
}

#[test]
fn test_move_index_left_shift() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.reserve(5);
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 0);
    assert_eq!(entries[0].key, 2);
    assert_eq!(entries[1].key, 1);
}

#[test]
fn test_move_index_right_shift() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.reserve(5);
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 2);
    assert_eq!(entries[1].key, 3);
    assert_eq!(entries[2].key, 2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.reserve(5);
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 2); // Should panic as index 2 does not exist
}

