// Answer 0

#[test]
fn test_lookup_existing_key() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    let index = hashmap.lookup(1);
    assert_eq!(index, 0);
}

#[test]
fn test_lookup_nonexistent_key() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(2);
    assert_eq!(index, 2); // since the index 2 will return default
}

#[test]
fn test_lookup_collision_resolution() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar { key: 7, value: 70 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(7);
    assert_eq!(index, 1); // should return the index of key 7
}

#[test]
#[should_panic(expected = "callers have to ensure map is allocated")]
fn test_lookup_panics_when_map_not_allocated() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: None,
    };

    hashmap.lookup(1);
}

