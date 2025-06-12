// Answer 0

fn test_decrement_indices() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            let entries = vec![
                Bucket { hash: HashValue(0), key: 0, value: 100 },
                Bucket { hash: HashValue(1), key: 1, value: 101 },
                Bucket { hash: HashValue(2), key: 2, value: 102 },
                Bucket { hash: HashValue(3), key: 3, value: 103 },
            ];
            Self { entries }
        }
    }

    let mut indices = vec![1, 2, 3, 4]; // Indices where start <= *i < end applies
    let start = 1;
    let end = 3;
    let mut entries = TestEntries::new();

    let mut ref_mut = RefMut::new(&mut indices, &mut entries.entries);
    ref_mut.decrement_indices(start, end);

    assert_eq!(indices, vec![0, 1, 2, 3]); // After decrementing, the indices should reflect the shift
}

fn test_decrement_indices_large() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            let buckets = 15; // Larger than half of the capacity
            let entries = (0..buckets)
                .map(|i| Bucket { hash: HashValue(i), key: i, value: i * 100 })
                .collect();
            Self { entries }
        }
    }

    let mut indices = vec![2, 3, 4, 5, 6, 7]; // Indices where start <= *i < end applies
    let start = 2;
    let end = 6;
    let mut entries = TestEntries::new();

    let mut ref_mut = RefMut::new(&mut indices, &mut entries.entries);
    ref_mut.decrement_indices(start, end);

    assert_eq!(indices, vec![2, 3, 4, 5, 5, 6]); // Ensure the correct indices after decrementing
}

fn test_decrement_indices_empty() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }
    }

    let mut indices = vec![]; // No indices
    let start = 0;
    let end = 0;
    let mut entries = TestEntries::new();

    let mut ref_mut = RefMut::new(&mut indices, &mut entries.entries);
    ref_mut.decrement_indices(start, end);

    assert_eq!(indices, vec![]); // No change in empty indices
}

