// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    
    // Filling the entries with dummy data
    index_map.entries.push(Bucket::new(0, 0));
    index_map.entries.push(Bucket::new(1, 1));
    index_map.entries.push(Bucket::new(2, 2));
    index_map.entries.push(Bucket::new(3, 3));
    
    // Swap two existing indices within bounds
    index_map.swap_indices(0, 1);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_with_invalid_indices() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    
    // Filling the entries with dummy data
    index_map.entries.push(Bucket::new(0, 0));
    index_map.entries.push(Bucket::new(1, 1));
    
    // These indices are valid but will cause panic based on the condition in the method implementation
    index_map.swap_indices(0, 0);
}

#[test]
fn test_swap_indices_with_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    
    // Filling with maximum capacity entries
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        index_map.entries.push(Bucket::new(i, i));
    }
    
    // Swap two indices that are valid and within capacity
    index_map.swap_indices(5, 10);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is")]
fn test_swap_indices_with_out_of_bounds_a() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    
    // Filling the entries with dummy data
    index_map.entries.push(Bucket::new(0, 0));
    index_map.entries.push(Bucket::new(1, 1));
    
    // Attempting to swap with index 'a' out of bounds
    index_map.swap_indices(2, 1);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is")]
fn test_swap_indices_with_out_of_bounds_b() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    
    // Filling the entries with dummy data
    index_map.entries.push(Bucket::new(0, 0));
    index_map.entries.push(Bucket::new(1, 1));
    
    // Attempting to swap with index 'b' out of bounds
    index_map.swap_indices(1, 2);
}

#[test]
fn test_swap_indices_with_non_adjacent_valid_indices() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    // Filling the entries with dummy data
    index_map.entries.push(Bucket::new(0, 0));
    index_map.entries.push(Bucket::new(1, 1));
    index_map.entries.push(Bucket::new(2, 2));
    
    // Swap two non-adjacent indices that are valid
    index_map.swap_indices(0, 2);
}

