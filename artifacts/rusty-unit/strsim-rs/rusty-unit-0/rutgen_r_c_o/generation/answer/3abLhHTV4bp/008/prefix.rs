// Answer 0

#[test]
fn test_grow_with_min_used_beyond_current_capacity() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_with_min_used_exactly_half_of_capacity() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 4]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_with_min_used_less_than_current_capacity() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: 5 }, GrowingHashmapMapElemChar::default(); 4]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_with_single_element_and_min_used() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: 5 }, GrowingHashmapMapElemChar::default()]),
    };
    hashmap.grow(1);
}

#[test]
fn test_grow_with_all_default_elements_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };
    hashmap.grow(1);
}

