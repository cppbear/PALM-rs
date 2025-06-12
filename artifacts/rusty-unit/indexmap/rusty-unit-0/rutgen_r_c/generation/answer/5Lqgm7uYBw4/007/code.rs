// Answer 0

#[test]
fn test_swap_indices_different_values() {
    struct TestEntries {
        data: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new(data: Vec<Bucket<usize, usize>>) -> Self {
            Self { data }
        }
    }

    let mut indices = hashbrown::HashMap::new();
    let mut entries = TestEntries::new(vec![
        Bucket { hash: HashValue(1), key: 0, value: 100 },
        Bucket { hash: HashValue(2), key: 1, value: 200 },
        Bucket { hash: HashValue(3), key: 2, value: 300 },
    ]);

    // Preparing indices to map correctly to entries
    indices.insert(1, 0);
    indices.insert(2, 1);
    indices.insert(3, 2);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries.data);

    // Swap valid indices
    ref_mut.swap_indices(0, 1);

    assert_eq!(ref_mut.entries[0].key, 1);
    assert_eq!(ref_mut.entries[1].key, 0);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_invalid_indices() {
    struct TestEntries {
        data: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new(data: Vec<Bucket<usize, usize>>) -> Self {
            Self { data }
        }
    }

    let mut indices = hashbrown::HashMap::new();
    let mut entries = TestEntries::new(vec![
        Bucket { hash: HashValue(1), key: 0, value: 100 },
        Bucket { hash: HashValue(2), key: 1, value: 200 },
    ]);

    // Only two entries, so this index will be out of bounds
    indices.insert(1, 0);
    indices.insert(2, 1);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries.data);

    // This should panic due to invalid indices
    ref_mut.swap_indices(0, 2);
}

