// Answer 0

fn test_grow_min_used_greater_than_new_size() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        mask: i32,
        used: i32,
        fill: i32,
        map: Option<Vec<GrowingHashmapMapElemChar>>,
    }

    impl Default for GrowingHashmapMapElemChar {
        fn default() -> Self {
            GrowingHashmapMapElemChar { key: '\0', value: 0 }
        }
    }

    let mut hashmap = GrowingHashmap {
        mask: 3, // initial size 4
        used: 2,
        fill: 2,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 'a', value: 1 },
            GrowingHashmapMapElemChar { key: 'b', value: 2 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(10); // new_size will be 8, which satisfies new_size > min_used
    assert_eq!(hashmap.mask, 7); // expect new mask to be 7 (new_size - 1)
    assert_eq!(hashmap.fill, hashmap.used); // fill should equal used
}

fn test_grow_map_not_initialized() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        mask: i32,
        used: i32,
        fill: i32,
        map: Option<Vec<GrowingHashmapMapElemChar>>,
    }

    impl Default for GrowingHashmapMapElemChar {
        fn default() -> Self {
            GrowingHashmapMapElemChar { key: '\0', value: 0 }
        }
    }

    let mut hashmap = GrowingHashmap {
        mask: 3, // initial size 4
        used: 2,
        fill: 2,
        map: None, // map not initialized
    };

    let result = std::panic::catch_unwind(|| {
        hashmap.grow(5);
    });
    
    assert!(result.is_err()); // expecting panic due to uninitialized map
}

fn test_grow_with_empty_map() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct GrowingHashmap {
        mask: i32,
        used: i32,
        fill: i32,
        map: Option<Vec<GrowingHashmapMapElemChar>>,
    }

    impl Default for GrowingHashmapMapElemChar {
        fn default() -> Self {
            GrowingHashmapMapElemChar { key: '\0', value: 0 }
        }
    }

    let mut hashmap = GrowingHashmap {
        mask: 3, 
        used: 1, // used is not 0
        fill: 1, 
        map: Some(vec![GrowingHashmapMapElemChar::default(); 4]), // empty but allocated
    };

    hashmap.map.as_mut().unwrap()[0].value = 1; // set value to non-default
    hashmap.grow(5); // should not panic
    assert_eq!(hashmap.mask, 7); // expect new mask to be 7 (new_size - 1)
    assert_eq!(hashmap.fill, hashmap.used); // fill should equal used
}

