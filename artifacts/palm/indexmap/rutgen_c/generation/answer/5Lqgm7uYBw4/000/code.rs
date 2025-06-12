// Answer 0

#[test]
fn test_swap_indices_equal_indices() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 0); // Should not panic or change anything
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 2);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_out_of_bounds() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 3); // Should panic due to out of bounds
}

#[test]
fn test_swap_indices_different_indices() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    indices.insert(0, 0);
    indices.insert(1, 1);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 1);
    assert_eq!(entries[0].key, 2);
    assert_eq!(entries[1].key, 1);
}

#[test]
fn test_swap_indices_same_indices() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    indices.insert(0, 0);
    indices.insert(1, 1);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(1, 1); // Should be no change
    assert_eq!(entries[1].key, 2);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_nonexistent_hash() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    indices.insert(0, 0);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 2); // Should panic due to nonexistent hash
}

