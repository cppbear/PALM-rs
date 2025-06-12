// Answer 0

#[test]
fn test_increment_indices_with_half_capacity() {
    let mut indices = vec![1, 2, 3, 4, 5, 6, 7]; // Sample indices
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    
    // Creating RefMut
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Set the start and end indices
    let start = 1;
    let end = 3; // This makes shifted_entries.len() == 2, which is half of a capacity of 4

    // Perform the increment
    ref_mut.increment_indices(start, end);

    // Check that the indices have been incremented correctly
    assert_eq!(indices, vec![1, 2, 4, 5, 6, 7]); // Indices 2 and 3 should have been incremented to 3 and 4
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_increment_indices_out_of_bounds() {
    let mut indices = vec![0, 1, 2, 3];
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Intentionally setting start and end out of bounds
    let start = 0;
    let end = 4; // This should panic, as entries only contain 3 elements

    // This will panic
    ref_mut.increment_indices(start, end);
}

