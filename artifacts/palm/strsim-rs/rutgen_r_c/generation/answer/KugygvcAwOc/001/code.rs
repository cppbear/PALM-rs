// Answer 0

#[test]
fn test_lookup_when_map_is_allocated_and_key_matches() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 0 }, // Default value
            GrowingHashmapMapElemChar { key: 2, value: 42 }, // Non-default value
        ]),
    };
    let result = hashmap.lookup(2);
    assert_eq!(result, 1); // Expect index 1 for the key 2 which has a non-default value
}

#[test]
fn test_lookup_when_map_is_allocated_and_entry_is_default() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 0 }, // Non-default value
            GrowingHashmapMapElemChar { key: 2, value: 0 }, // Default value
        ]),
    };
    let result = hashmap.lookup(2);
    assert_eq!(result, 1); // Expect index 1 because value is default
}

#[test]
fn test_lookup_when_map_is_allocated_and_key_not_found() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 0 }, // Non-default value
            GrowingHashmapMapElemChar { key: 2, value: 42 }, // Non-default value
        ]),
    };
    let result = hashmap.lookup(3); // Key 3 is not present
    assert_eq!(result, 0); // Expect to return index 0, which has a non-default value
}

