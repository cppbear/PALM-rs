// Answer 0

#[test]
fn test_move_index_from_0_to_1() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 1);
}

#[test]
fn test_move_index_from_1_to_0() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(0), key: 0, value: 0 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 0);
}

#[test]
fn test_move_index_from_0_to_0() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

#[test]
fn test_move_index_from_2_to_2() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(2), key: 2, value: 2 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

#[test]
fn test_move_index_from_1_to_1() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 1 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

#[test]
fn test_move_index_from_3_to_3() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(3), key: 3, value: 3 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

#[test]
fn test_move_index_from_4_to_4() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(4), key: 4, value: 4 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

