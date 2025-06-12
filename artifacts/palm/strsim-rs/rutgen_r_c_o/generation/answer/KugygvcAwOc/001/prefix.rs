// Answer 0

#[test]
fn test_lookup_first_slot_empty() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 0,
        mask: 31,
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: 0 }; 32]),
    };
    let result = hashmap.lookup(0);
}

#[test]
fn test_lookup_first_slot_default_value() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 0,
        mask: 31,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 2 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(3);
}

#[test]
fn test_lookup_with_collision() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 1,
        fill: 1,
        mask: 31,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 2 },
            GrowingHashmapMapElemChar { key: 3, value: 0 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(3);
}

#[test]
fn test_lookup_non_empty_map() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 2,
        fill: 3,
        mask: 31,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 42 },
            GrowingHashmapMapElemChar { key: 1, value: 43 },
            GrowingHashmapMapElemChar { key: 2, value: 44 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(1);
}

#[test]
fn test_lookup_to_trigger_loop() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 1,
        fill: 1,
        mask: 31,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 },
            GrowingHashmapMapElemChar { key: 1, value: 0 },
            GrowingHashmapMapElemChar { key: 2, value: 0 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let result = hashmap.lookup(2);
}

