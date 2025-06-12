// Answer 0

#[test]
fn test_get_no_elements() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    assert_eq!(hashmap.get(1), 0); // Default value for i32
}

#[test]
fn test_get_with_default_elements() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: 10 }]),
    };
    assert_eq!(hashmap.get(1), 10); // Expected value for key 1
}

#[test]
fn test_get_with_unused_key() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 2, value: 20 }]),
    };
    assert_eq!(hashmap.get(1), 0); // Default value, key 1 does not exist
}

#[test]
fn test_get_with_multiple_elements() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 2,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
        ]),
    };
    assert_eq!(hashmap.get(2), 20); // Expected value for key 2
}

#[test]
fn test_get_with_non_existent_key() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 2,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
        ]),
    };
    assert_eq!(hashmap.get(2), 0); // Default value, key 2 does not exist
}

