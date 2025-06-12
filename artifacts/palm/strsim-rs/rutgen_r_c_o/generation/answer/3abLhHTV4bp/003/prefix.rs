// Answer 0

#[test]
fn test_grow_min_used_equal_new_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: Default::default() }; 8]),
    };
    hashmap.grow(8);
}

#[test]
fn test_grow_min_used_greater_new_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 7,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: Default::default() }; 8]),
    };
    hashmap.grow(16);
}

#[test]
fn test_grow_min_used_less_new_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 1 }; 4]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_with_default_values_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 1 }, GrowingHashmapMapElemChar { key: 1, value: Default::default() }]),
    };
    hashmap.grow(4);
}

#[test]
fn test_grow_with_non_default_values_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 1 }, GrowingHashmapMapElemChar { key: 1, value: 2 }]),
    };
    hashmap.grow(4);
}

#[test]
fn test_grow_empty_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![]),
    };
    hashmap.grow(4);
}

