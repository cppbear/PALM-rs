// Answer 0

#[test]
fn test_get_range_valid_range() {
    struct TestHashBuilder;

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    
    let result = index_map.get_range(0..2);
    assert!(result.is_some());
    let slice = result.unwrap();
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, 10);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[1].value, 20);
}

#[test]
fn test_get_range_exclusive_bounds() {
    struct TestHashBuilder;

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    
    let result = index_map.get_range(1..3);
    assert!(result.is_some());
    let slice = result.unwrap();
    assert_eq!(slice.entries[0].key, 2);
    assert_eq!(slice.entries[0].value, 20);
    assert_eq!(slice.entries[1].key, 3);
    assert_eq!(slice.entries[1].value, 30);
}

#[test]
fn test_get_range_invalid_bounds() {
    struct TestHashBuilder;

    let mut index_map: IndexMap<i32, i32, TestHashBuilder> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: TestHashBuilder,
    };

    index_map.insert(1, 10);
    
    let result = index_map.get_range(1..0);
    assert!(result.is_none());
    
    let result_out_of_bounds = index_map.get_range(5..10);
    assert!(result_out_of_bounds.is_none());
}

