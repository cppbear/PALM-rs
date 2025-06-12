// Answer 0

#[test]
fn test_move_index_valid_case() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
        Bucket { hash: HashValue(4), key: 3, value: 3 },
        Bucket { hash: HashValue(5), key: 4, value: 4 },
        Bucket { hash: HashValue(6), key: 5, value: 5 },
        Bucket { hash: HashValue(7), key: 6, value: 6 },
        Bucket { hash: HashValue(8), key: 7, value: 7 },
        Bucket { hash: HashValue(9), key: 8, value: 8 },
        Bucket { hash: HashValue(10), key: 9, value: 9 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 5);
}

#[test]
#[should_panic]
fn test_move_index_from_equals_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
        Bucket { hash: HashValue(4), key: 3, value: 3 },
        Bucket { hash: HashValue(5), key: 4, value: 4 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(3, 3);
}

#[test]
fn test_move_index_from_greater_than_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
        Bucket { hash: HashValue(4), key: 3, value: 3 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(3, 1);
}

#[test]
fn test_move_index_from_less_than_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
        Bucket { hash: HashValue(3), key: 2, value: 2 },
        Bucket { hash: HashValue(4), key: 3, value: 3 },
        Bucket { hash: HashValue(5), key: 4, value: 4 },
        Bucket { hash: HashValue(6), key: 5, value: 5 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 4);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 3);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 0 },
        Bucket { hash: HashValue(2), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 1);
}

