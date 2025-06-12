// Answer 0

#[test]
fn test_grow_new_size_equal_min_used() {
    let mut hashmap = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar { key: 2, value: 43 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(4);
    
    assert_eq!(hashmap.mask, 3); // 4 - 1
    assert_eq!(hashmap.used, 2);
}

#[test]
fn test_grow_new_size_less_than_min_used() {
    let mut hashmap = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar { key: 2, value: 43 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(6);
    
    assert_eq!(hashmap.mask, 7); // Grows to the next power of 2 (8 - 1)
    assert_eq!(hashmap.used, 2);
}

#[test]
#[should_panic(expected = "callers have to ensure map is allocated")]
fn test_grow_map_not_allocated() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    hashmap.grow(4);
}

#[test]
fn test_grow_elem_value_default() {
    let mut hashmap = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar { key: 2, value: 43 },
            GrowingHashmapMapElemChar::default(), // value is default
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(4);
    
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.used, 2);
}

#[test]
fn test_grow_elem_in_old_map_false() {
    let mut hashmap = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(), // No elements with non-default values
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(4);
    
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.used, 0);
}

