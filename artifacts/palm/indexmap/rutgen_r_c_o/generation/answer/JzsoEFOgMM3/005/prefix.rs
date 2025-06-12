// Answer 0

#[test]
fn test_decrement_indices_empty_range() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0);
}

#[test]
fn test_decrement_indices_half_capacity() {
    let capacity = 4;
    let mut indices = hash_table::HashTable::new();
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 100 },
        Bucket { hash: HashValue(1), key: 1, value: 101 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 2);
}

#[test]
fn test_decrement_indices_capacity_exceeds_half() {
    let capacity = 4;
    let mut indices = hash_table::HashTable::new();
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 100 },
        Bucket { hash: HashValue(1), key: 1, value: 101 },
        Bucket { hash: HashValue(2), key: 2, value: 102 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 3);
}

#[test]
fn test_decrement_indices_out_of_bounds() {
    let capacity = 4;
    let mut indices = hash_table::HashTable::new();
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 100 },
        Bucket { hash: HashValue(1), key: 1, value: 101 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(2, 2);
}

