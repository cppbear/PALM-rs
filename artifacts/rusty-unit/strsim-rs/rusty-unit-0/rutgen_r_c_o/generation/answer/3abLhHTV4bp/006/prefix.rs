// Answer 0

#[test]
fn test_grow_case_valid() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: 42 }, GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default()]),
    };
    hashmap.grow(5);
}

#[test]
fn test_grow_case_edge_min_used() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 5,
        map: Some(vec![GrowingHashmapMapElemChar { key: 2, value: 99 }, GrowingHashmapMapElemChar { key: 3, value: 100 }, GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default()]),
    };
    hashmap.grow(6);
}

#[test]
fn test_grow_case_panic_no_allocation() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: None,
    };
    hashmap.grow(7);
}

#[test]
fn test_grow_case_exceeding_used() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 1,
        mask: 3,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 1 }, GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default(), GrowingHashmapMapElemChar::default()]),
    };
    hashmap.grow(1);
}

#[test]
fn test_grow_case_with_multiple_elements() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 2 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 3, value: 3 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(10);
}

