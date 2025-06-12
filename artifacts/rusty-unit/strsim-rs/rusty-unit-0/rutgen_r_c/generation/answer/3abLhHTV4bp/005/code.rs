// Answer 0

#[test]
fn test_grow_increase_size() {
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

    hashmap.grow(3);

    assert_eq!(hashmap.mask, 7); // new size should be 8
    assert_eq!(hashmap.used, 2);  // used should remain unchanged
}

#[test]
#[should_panic]
fn test_grow_panic_on_empty_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    hashmap.grow(1);
}

#[test]
fn test_grow_with_no_empty_elems() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(4);

    assert_eq!(hashmap.mask, 7); // new mask after growing
    assert_eq!(hashmap.used, 3);  // used count should remain the same
}

#[test]
#[should_panic]
fn test_grow_panic_on_used_unexpected_state() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(2);
}

