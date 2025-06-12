// Answer 0

fn test_swap_indices_valid() {
    let mut indices: Indices = hash_table::HashTable::new();
    let hash_a = HashValue(1);
    let hash_b = HashValue(2);
    
    indices.insert(hash_a.get(), 0);
    indices.insert(hash_b.get(), 1);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_a, key: 0, value: 100 },
        Bucket { hash: hash_b, key: 1, value: 200 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

fn test_swap_indices_same_hash() {
    let mut indices: Indices = hash_table::HashTable::new();
    let hash_a = HashValue(1);
    
    indices.insert(hash_a.get(), 0);
    indices.insert(hash_a.get(), 1);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_a, key: 0, value: 100 },
        Bucket { hash: hash_a, key: 1, value: 200 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

fn test_swap_indices_equal_values() {
    let mut indices: Indices = hash_table::HashTable::new();
    let hash_a = HashValue(3);
    let hash_b = HashValue(4);
    
    indices.insert(hash_a.get(), 0);
    indices.insert(hash_b.get(), 1);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_a, key: 0, value: 100 },
        Bucket { hash: hash_b, key: 1, value: 100 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

fn test_swap_indices_multiple_entries() {
    let mut indices: Indices = hash_table::HashTable::new();
    let hash_a = HashValue(5);
    let hash_b = HashValue(6);
    let hash_c = HashValue(7);
    
    indices.insert(hash_a.get(), 0);
    indices.insert(hash_b.get(), 1);
    indices.insert(hash_c.get(), 2);

    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: hash_a, key: 0, value: 300 },
        Bucket { hash: hash_b, key: 1, value: 400 },
        Bucket { hash: hash_c, key: 2, value: 500 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(1, 2);
}

