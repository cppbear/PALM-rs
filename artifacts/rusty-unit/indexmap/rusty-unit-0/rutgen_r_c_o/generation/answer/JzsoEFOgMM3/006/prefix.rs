// Answer 0

#[test]
fn test_decrement_indices_edge_case_empty_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0);
}

#[test]
fn test_decrement_indices_exact_half_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(0);
    indices.insert(1);
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 2);
}

#[test]
fn test_decrement_indices_no_change() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(2);
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 1);
} 

#[test]
fn test_decrement_indices_full_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(0);
    indices.insert(1);
    indices.insert(2);
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(2, 3);
}

#[test]
fn test_decrement_indices_with_multiple_entries() {
    let mut indices = hash_table::HashTable::with_capacity(4);
    indices.insert(0);
    indices.insert(1);
    indices.insert(2);
    indices.insert(3);
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 3);
}

