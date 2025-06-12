// Answer 0

#[test]
fn test_first_mut_empty() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::from(Vec::new()),
                },
                hash_builder: (),
            }
        }
    }

    let mut map: TestMap<i32, i32> = TestMap {
        entries: Vec::new(),
    };
    let result = map.first_mut();
    assert_eq!(result, None);
}

#[test]
fn test_first_mut_single_entry() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::from(vec![Bucket {
                        hash: HashValue::new(0),
                        key: 1,
                        value: 10,
                    }]),
                },
                hash_builder: (),
            }
        }
    }

    let mut map: TestMap<i32, i32> = TestMap::new();
    let result = map.first_mut();
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(*key, 1);
    *value += 5;
    assert_eq!(*value, 15);
}

#[test]
fn test_first_mut_multiple_entries() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn new() -> Self {
            Self {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::from(vec![
                        Bucket {
                            hash: HashValue::new(1),
                            key: 1,
                            value: 10,
                        },
                        Bucket {
                            hash: HashValue::new(2),
                            key: 2,
                            value: 20,
                        },
                    ]),
                },
                hash_builder: (),
            }
        }
    }

    let mut map: TestMap<i32, i32> = TestMap::new();
    let result = map.first_mut();
    assert!(result.is_some());
    let (key, value) = result.unwrap();
    assert_eq!(*key, 1);
    *value += 5;
    assert_eq!(*value, 15);
}

