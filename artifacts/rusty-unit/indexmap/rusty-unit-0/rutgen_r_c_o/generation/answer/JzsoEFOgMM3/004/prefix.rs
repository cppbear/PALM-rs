// Answer 0

#[test]
fn test_decrement_indices_case_1() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, usize> = vec![Bucket { hash: HashValue(0), key: 0, value: 10 }, Bucket { hash: HashValue(1), key: 1, value: 20 }];
    let capacity = 4;
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 2);
}

#[test]
fn test_decrement_indices_case_2() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, usize> = vec![Bucket { hash: HashValue(0), key: 0, value: 10 }, Bucket { hash: HashValue(1), key: 1, value: 20 },
        Bucket { hash: HashValue(2), key: 2, value: 30 }, Bucket { hash: HashValue(3), key: 3, value: 40 }];
    let capacity = 4;
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 3);
}

#[test]
fn test_decrement_indices_edge_case() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Entries<usize, usize> = vec![Bucket { hash: HashValue(0), key: 0, value: 10 }; 6];
    let capacity = 4;
    for i in 0..capacity {
        indices.insert(i);
    }
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(2, 6);
}

