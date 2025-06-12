// Answer 0

#[test]
fn test_lookup_key_found() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }, GrowingHashmapMapElemChar { key: 1, value: 42 }]),
    };
    let index = hashmap.lookup(1);
    assert_eq!(index, 1);
}

#[test]
fn test_lookup_key_not_found_default() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };
    let index = hashmap.lookup(2);
    assert_eq!(index, 0);
}

#[test]
fn test_lookup_key_collisions() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 1, value: 0 },
            GrowingHashmapMapElemChar { key: 3, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 0 },
        ]),
    };
    let index = hashmap.lookup(3);
    assert_eq!(index, 2);
}

#[test]
fn test_lookup_key_with_empty_slot() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 2, value: 5 },
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 0, value: 0 },
        ]),
    };
    let index = hashmap.lookup(2);
    assert_eq!(index, 0);
}

