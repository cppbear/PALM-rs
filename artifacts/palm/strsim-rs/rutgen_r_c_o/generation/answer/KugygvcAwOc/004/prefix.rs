// Answer 0

#[test]
fn test_lookup_with_default_value_and_key() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 0,
        mask: 0b111, // Example mask value
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar::default(), // Default value
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(), // Default value
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar::default(), // Default value
            GrowingHashmapMapElemChar { key: 5, value: 50 },
        ]),
    };
    let _ = hashmap.lookup(0);
}

#[test]
fn test_lookup_with_used_key() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 3,
        fill: 3,
        mask: 0b111,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 5, value: 50 },
        ]),
    };
    let _ = hashmap.lookup(2);
}

#[test]
fn test_lookup_for_non_existing_key() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 4,
        fill: 4,
        mask: 0b111,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar { key: 5, value: 50 },
            GrowingHashmapMapElemChar { key: 6, value: 60 },
        ]),
    };
    let _ = hashmap.lookup(7);
}

#[test]
fn test_lookup_with_collision_resolution() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 3,
        fill: 3,
        mask: 0b111,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            // Collision with existing value
            GrowingHashmapMapElemChar { key: 1, value: 25 },
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar { key: 5, value: 50 },
            GrowingHashmapMapElemChar { key: 6, value: 60 },
        ]),
    };
    let _ = hashmap.lookup(1);
}

#[test]
fn test_lookup_for_edge_key_zero() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 1,
        fill: 1,
        mask: 0b111,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 100 }, // Key is zero
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let _ = hashmap.lookup(0);
}

#[test]
fn test_lookup_for_maximum_key_value() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 3,
        fill: 3,
        mask: 0b111,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 4294967295, value: 100 }, // Maximum key value
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let _ = hashmap.lookup(4294967295);
}

