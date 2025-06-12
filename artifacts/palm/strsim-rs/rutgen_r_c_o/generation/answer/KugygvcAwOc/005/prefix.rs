// Answer 0

#[test]
fn test_lookup_with_allocated_map_and_key_present() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 1,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 },
            GrowingHashmapMapElemChar { key: 2, value: 10 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(1);
}

#[test]
fn test_lookup_with_allocated_map_and_key_absent() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 1,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 },
            GrowingHashmapMapElemChar { key: 3, value: 15 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(2);
}

#[test]
fn test_lookup_with_allocated_map_and_value_default() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 2,
        fill: 3,
        mask: 4,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 3, value: 20 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(2);
}

#[test]
fn test_lookup_with_key_collisions() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 2,
        fill: 4,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 },
            GrowingHashmapMapElemChar { key: 2, value: 10 },
            GrowingHashmapMapElemChar { key: 5, value: 15 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(5);
}

#[test]
fn test_lookup_with_unallocated_map() {
    let hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let result = std::panic::catch_unwind(|| {
        hashmap.lookup(1);
    });
    assert!(result.is_err());
}

