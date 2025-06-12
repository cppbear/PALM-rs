// Answer 0

#[test]
fn test_get_mut_initial_allocation() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let value = hashmap.get_mut(1);
    *value = 10;

    assert_eq!(hashmap.get(1), 10);
}

#[test]
fn test_get_mut_reallocation_and_value_update() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    for i in 1..6 {
        let value = hashmap.get_mut(i);
        *value = i * 10;
    }

    // Verify that the values are correctly updated.
    for i in 1..6 {
        assert_eq!(hashmap.get(i), i * 10);
    }
}

#[test]
fn test_get_mut_resize() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Filling up the hashmap to trigger resizing.
    for i in 1..=6 {
        let value = hashmap.get_mut(i);
        *value = i * 20;
    }

    // Check that values have persisted after resizing.
    for i in 1..=6 {
        assert_eq!(hashmap.get(i), i * 20);
    }
}

#[test]
#[should_panic]
fn test_get_mut_panic_empty_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let _value = hashmap.get_mut(1);
    *(_value); // Trigger panic by accessing a default value.
}

