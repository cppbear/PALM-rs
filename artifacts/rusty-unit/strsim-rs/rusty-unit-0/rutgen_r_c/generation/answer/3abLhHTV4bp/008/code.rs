// Answer 0

#[test]
fn test_grow_increases_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::<i32>::default(); 1]),
    };
    hashmap.grow(2);
    assert_eq!(hashmap.mask, 1);
}

#[test]
fn test_grow_does_not_panic_when_map_is_allocated() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::<i32>::default(); 1]),
    };
    let result = std::panic::catch_unwind(|| {
        hashmap.grow(1);
    });
    assert!(result.is_ok());
}

#[test]
fn test_grow_correctly_moves_existing_elements() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 0,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(2);
    assert_eq!(hashmap.map.as_ref().unwrap()[0].key, 1);
    assert_eq!(hashmap.map.as_ref().unwrap()[0].value, 10);
}

#[test]
fn test_grow_with_no_elements_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::<i32>::default(); 1]),
    };
    hashmap.grow(1);
    assert_eq!(hashmap.fill, 0);
    assert_eq!(hashmap.used, 0);
}

