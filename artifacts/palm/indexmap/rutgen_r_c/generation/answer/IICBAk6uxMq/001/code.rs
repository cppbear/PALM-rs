// Answer 0

#[test]
fn test_swap_indices_valid() {
    let mut map_core: IndexMapCore<usize, usize> = IndexMapCore::new();
    map_core.entries.push(Bucket { key: 1, value: 10 });
    map_core.entries.push(Bucket { key: 2, value: 20 });
    
    map_core.swap_indices(0, 1);
    
    assert_eq!(map_core.entries[0].key, 2);
    assert_eq!(map_core.entries[1].key, 1);
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_out_of_bounds_a() {
    let mut map_core: IndexMapCore<usize, usize> = IndexMapCore::new();
    map_core.entries.push(Bucket { key: 1, value: 10 });
    map_core.entries.push(Bucket { key: 2, value: 20 });
    
    map_core.swap_indices(2, 1); // 'a' is out of bounds
}

#[test]
#[should_panic(expected = "indices not found")]
fn test_swap_indices_out_of_bounds_b() {
    let mut map_core: IndexMapCore<usize, usize> = IndexMapCore::new();
    map_core.entries.push(Bucket { key: 1, value: 10 });
    map_core.entries.push(Bucket { key: 2, value: 20 });
    
    map_core.swap_indices(0, 2); // 'b' is out of bounds
}

#[test]
fn test_swap_indices_same_index() {
    let mut map_core: IndexMapCore<usize, usize> = IndexMapCore::new();
    map_core.entries.push(Bucket { key: 1, value: 10 });
    
    map_core.swap_indices(0, 0); // swapping the same index
    
    assert_eq!(map_core.entries[0].key, 1);
}

#[test]
#[should_panic(expected = "cannot swap indices as they are equal and within bounds")]
fn test_swap_indices_with_equal_indices() {
    let mut map_core: IndexMapCore<usize, usize> = IndexMapCore::new();
    map_core.entries.push(Bucket { key: 1, value: 10 });
    map_core.entries.push(Bucket { key: 2, value: 20 });
    
    map_core.swap_indices(0, 0); // should not panic but do nothing
    assert_eq!(map_core.entries[0].key, 1);
}

