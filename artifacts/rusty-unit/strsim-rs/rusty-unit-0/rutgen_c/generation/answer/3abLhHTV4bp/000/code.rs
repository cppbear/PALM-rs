// Answer 0

#[test]
fn test_grow_increases_map_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };

    hashmap.grow(1);
    assert_eq!(hashmap.mask, 3); // Should grow from mask 1 to 3
}

#[test]
fn test_grow_with_empty_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };

    hashmap.grow(2);
    assert_eq!(hashmap.mask, 3); // Grows to size 4
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 4); // Map size should be 4
}

#[test]
fn test_grow_maintains_elements() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(4);
    assert_eq!(hashmap.map.as_ref().unwrap()[0].key, 1);
    assert_eq!(hashmap.map.as_ref().unwrap()[0].value, 10);
    assert_eq!(hashmap.map.as_ref().unwrap()[1].key, 2);
    assert_eq!(hashmap.map.as_ref().unwrap()[1].value, 20);
}

#[test]
fn test_grow_no_elements() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };

    hashmap.grow(0);
    assert_eq!(hashmap.mask, 1); // No need to grow, mask should remain the same
}

#[test]
fn test_grow_empty_to_non_empty() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };

    hashmap.grow(1);
    assert_eq!(hashmap.mask, 3); // Grows as required
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 4); // Map size should be 4
}

