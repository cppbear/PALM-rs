// Answer 0

#[test]
fn test_lookup_key_found() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar { key: 2, value: 0 },
            GrowingHashmapMapElemChar { key: 3, value: 0 },
        ]),
    };

    let result = hashmap.lookup(1);
    assert_eq!(result, 1);
}

#[test]
fn test_lookup_key_not_found() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 1, value: 0 },
            GrowingHashmapMapElemChar { key: 2, value: 0 },
            GrowingHashmapMapElemChar { key: 3, value: 0 },
        ]),
    };

    let result = hashmap.lookup(2);
    assert_eq!(result, 2);
}

#[should_panic(expected = "callers have to ensure map is allocated")]
#[test]
fn test_lookup_map_not_allocated() {
    let hashmap = GrowingHashmapChar::<i32> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let _ = hashmap.lookup(0);
}

#[test]
fn test_lookup_with_collision() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 3, value: 100 },
            GrowingHashmapMapElemChar { key: 1, value: 0 },
            GrowingHashmapMapElemChar { key: 5, value: 42 },
        ]),
    };

    let result = hashmap.lookup(5);
    assert_eq!(result, 3);
}

