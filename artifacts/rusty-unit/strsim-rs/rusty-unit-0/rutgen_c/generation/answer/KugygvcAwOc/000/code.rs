// Answer 0

#[test]
fn test_lookup_existing_key() {
    struct TestValue(u32);
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 1,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValue(10),
            },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(1);
    assert_eq!(index, 0);
}

#[test]
fn test_lookup_default_key() {
    struct TestValue(u32);
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 0,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(2);
    assert_eq!(index, 0);
}

#[test]
fn test_lookup_collision_resolution() {
    struct TestValue(u32);
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 2,
        fill: 4,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValue(10),
            },
            GrowingHashmapMapElemChar {
                key: 2,
                value: TestValue(20),
            },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(2);
    assert_eq!(index, 1);
}

#[test]
fn test_lookup_nonexistent_key() {
    struct TestValue(u32);
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 1,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValue(10),
            },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let index = hashmap.lookup(3);
    assert_eq!(index, 1);
}

