// Answer 0

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_from() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 1); // from is out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 2); // to is out of bounds
}

#[test]
fn test_move_index_same_position() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0); // from == to, no panic but should not change the state
}

#[test]
fn test_move_index_from_to_same() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 1); // from == to, no panic but should not change the state
}

#[test]
fn test_move_index_decrement() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 0); // Move the second entry to the first position

    assert_eq!(entries[0].key, 1); // key of the second entry
    assert_eq!(entries[1].key, 0); // key of the first entry
}

#[test]
fn test_move_index_increment() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 1); // Move the first entry to the second position

    assert_eq!(entries[0].key, 1); // key of the second entry
    assert_eq!(entries[1].key, 0); // key of the first entry
}

