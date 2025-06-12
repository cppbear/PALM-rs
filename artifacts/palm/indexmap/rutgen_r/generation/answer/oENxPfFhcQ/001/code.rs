// Answer 0

fn test_new_with_valid_range() {
    use indexmap::IndexMap;
    use std::ops::RangeBounds;

    struct TestReplacer;
    let mut map = IndexMap::new();
    map.insert(0, "a");
    map.insert(1, "b");
    map.insert(2, "c");
    
    // Valid range and replace_with
    {
        let range: std::ops::Range<usize> = 0..2;
        let replace_with = TestReplacer;

        let result = new(&mut map, range, replace_with);

        // Check the expected structure
        assert_eq!(map.len(), 1); // Expecting that the first two elements are drained
        assert_eq!(result.tail.len(), 1); // Remaining elements in tail
        assert_eq!(result.drain.len(), 2); // Drained elements
    }
}

fn test_new_with_empty_map() {
    use indexmap::IndexMap;

    struct TestReplacer;
    let mut map = IndexMap::new();
    
    // Valid range but should handle empty map
    {
        let range: std::ops::Range<usize> = 0..1;
        let replace_with = TestReplacer;

        let result = new(&mut map, range, replace_with);

        // Check the expected structure
        assert_eq!(map.len(), 0); // The map should still be empty
        assert!(result.tail.is_empty()); // Tail should be empty
        assert!(result.drain.is_empty()); // There should be nothing drained
    }
}

fn test_new_with_exceeding_range() {
    use indexmap::IndexMap;

    struct TestReplacer;
    let mut map = IndexMap::new();
    map.insert(0, "a");
    
    // Range exceeds current map size
    {
        let range: std::ops::Range<usize> = 0..5;
        let replace_with = TestReplacer;

        let result = new(&mut map, range, replace_with);

        // Check the expected structure
        assert_eq!(map.len(), 0); // The map should be empty after being drained
        assert_eq!(result.tail.len(), 0); // Tail should be empty
        assert_eq!(result.drain.len(), 1); // Drained the one element we had
    }
}

