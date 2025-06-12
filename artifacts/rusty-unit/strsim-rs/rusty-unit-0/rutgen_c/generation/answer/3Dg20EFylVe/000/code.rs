// Answer 0

#[test]
fn test_get_with_empty_map() {
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: 0 }
        }
    }

    let hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let result = hashmap.get(1);
    assert_eq!(result.data, 0);
}

#[test]
fn test_get_with_non_empty_map() {
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: -1 }
        }
    }

    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar {
            key: 1,
            value: TestValue { data: 42 },
        }]),
    };

    let result = hashmap.get(1);
    assert_eq!(result.data, 42);

    let default_result = hashmap.get(2);
    assert_eq!(default_result.data, -1);
}

