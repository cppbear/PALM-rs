// Answer 0

#[test]
fn test_sort_by_cached_key_empty_map() {
    struct KeyValuePair {
        key: i32,
        value: String,
    }

    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.sort_by_cached_key(|k, v| k); // Should work without panicking
    assert_eq!(map.as_slice().len(), 0); // Ensure the map is still empty
}

#[test]
fn test_sort_by_cached_key_single_entry() {
    struct KeyValuePair {
        key: i32,
        value: String,
    }

    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.sort_by_cached_key(|k, v| *k); // Should work without panicking
    assert_eq!(map.as_slice(), &[(1, "one".to_string())]); // Check single entry remains unchanged
}

#[test]
fn test_sort_by_cached_key_sorted_entries() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    map.sort_by_cached_key(|k, v| *k); // Should work without panicking
    assert_eq!(map.as_slice(), &[(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]); // Check order
}

#[test]
fn test_sort_by_cached_key_reverse_order() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(3, "three".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    map.sort_by_cached_key(|k, v| *k); // Should work without panicking
    assert_eq!(map.as_slice(), &[(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]); // Check sorting
}

#[test]
fn test_sort_by_cached_key_custom_sort_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(3, "three".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    map.sort_by_cached_key(|_k, v| v.len()); // Sort by length of the value
    assert_eq!(map.as_slice(), &[(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]); // Check length-based sorting
}

#[test]
fn test_sort_by_cached_key_panic_condition() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    // Introduce a sort key closure that panics
    let result = std::panic::catch_unwind(|| {
        map.sort_by_cached_key(|k, _| {
            if *k == 2 {
                panic!("panic triggered for key 2");
            }
            *k
        });
    });

    assert!(result.is_err()); // Check that the panic occurred
}

