// Answer 0

fn test_decrement_indices_large_shift() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
        Bucket { hash: HashValue(4), key: 4, value: "d" }
    ];

    // Initialize indices to simulate a scenario
    // where the length of shifted entries is greater than half the capacity of indices.
    for i in 0..8 {
        indices.insert(i, i);
    }

    // Create a mutable reference to entries
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Here, start is 1 and end is 4, which means we're shifting indices for entries 1 to 3.
    ref_mut.decrement_indices(1, 4);

    // Verify that indices were decremented correctly
    assert_eq!(indices.get(&1), None);
    assert_eq!(indices.get(&2), Some(&1));
    assert_eq!(indices.get(&3), Some(&2));
    assert_eq!(indices.get(&4), Some(&3));
    assert_eq!(indices.get(&5), Some(&4));
}

fn test_decrement_indices_no_shift() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries = vec![
        Bucket { hash: HashValue(10), key: 10, value: "x" },
        Bucket { hash: HashValue(20), key: 20, value: "y" },
        Bucket { hash: HashValue(30), key: 30, value: "z" }
    ];

    // Initialize the HashMap with a range where start == end
    for i in 0..5 {
        indices.insert(i, i);
    }

    // Create a mutable reference to entries
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // The range start = 3, end = 3 means no entries to shift
    ref_mut.decrement_indices(3, 3);

    // Assert the indices remain unchanged
    assert_eq!(indices.get(&0), Some(&0));
    assert_eq!(indices.get(&1), Some(&1));
    assert_eq!(indices.get(&2), Some(&2));
    assert_eq!(indices.get(&3), Some(&3));
    assert_eq!(indices.get(&4), Some(&4));
}

fn test_decrement_indices_edge_case() {
    let mut indices = hashbrown::HashMap::new();
    let mut entries = vec![
        Bucket { hash: HashValue(100), key: 100, value: "first" },
        Bucket { hash: HashValue(200), key: 200, value: "second" },
    ];

    // Initialize the HashMap to control the indices
    for i in 0..3 {
        indices.insert(i, i);
    }

    // We are going to test the edge case where start and end are at the boundaries
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // The range start = 0, end = 2 indicates that we are shifting entries 0 to 1
    ref_mut.decrement_indices(0, 2);

    // Assert that indices were decremented correctly
    assert_eq!(indices.get(&0), None);
    assert_eq!(indices.get(&1), Some(&0));
    assert_eq!(indices.get(&2), Some(&1));
}

