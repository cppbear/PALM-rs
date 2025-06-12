// Answer 0

#[test]
fn test_reserve_entries_with_exact_capacity() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let initial_capacity = 5;
    entries.reserve_exact(initial_capacity);
    
    let additional = 5;
    let try_capacity = additional + entries.len(); // try_add will be equal to additional due to entries.len() being initial_capacity.
    
    reserve_entries(&mut entries, additional, try_capacity);
    
    assert_eq!(entries.len(), initial_capacity + additional);
}

#[test]
#[should_panic]
fn test_reserve_entries_exceeding_max_capacity() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let initial_capacity = 5;
    entries.reserve_exact(initial_capacity);
    
    let additional = 10;
    let try_capacity = usize::MAX; // Exceeds the maximum capacity of IndexMapCore, should cause a panic in reserve_exact.

    reserve_entries(&mut entries, additional, try_capacity);
}

#[test]
fn test_reserve_entries_with_no_additional_capacity() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let initial_capacity = 5;
    entries.reserve_exact(initial_capacity);
    
    let additional = 0; // No additional space requested
    let try_capacity = initial_capacity; // try_add will be 0 in this case.

    reserve_entries(&mut entries, additional, try_capacity);
    
    assert_eq!(entries.len(), initial_capacity); // Should remain unchanged
}

