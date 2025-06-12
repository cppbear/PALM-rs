// Answer 0

#[test]
fn test_binary_search_keys_found() {
    struct KeyValue {
        key: i32,
    }

    impl Ord for KeyValue {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for KeyValue {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for KeyValue {}

    impl PartialEq for KeyValue {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    let keys = vec![
        KeyValue { key: 1 },
        KeyValue { key: 2 },
        KeyValue { key: 3 },
        KeyValue { key: 4 },
        KeyValue { key: 5 },
    ];

    let result = keys.binary_search_keys(&KeyValue { key: 3 });
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_keys_not_found() {
    struct KeyValue {
        key: i32,
    }

    impl Ord for KeyValue {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for KeyValue {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for KeyValue {}

    impl PartialEq for KeyValue {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    let keys = vec![
        KeyValue { key: 1 },
        KeyValue { key: 2 },
        KeyValue { key: 3 },
        KeyValue { key: 4 },
        KeyValue { key: 5 },
    ];

    let result = keys.binary_search_keys(&KeyValue { key: 6 });
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_keys_empty() {
    struct KeyValue {
        key: i32,
    }

    impl Ord for KeyValue {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for KeyValue {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for KeyValue {}

    impl PartialEq for KeyValue {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    let keys: Vec<KeyValue> = vec![];
    let result = keys.binary_search_keys(&KeyValue { key: 1 });
    assert_eq!(result, Err(0));
}

#[test]
#[should_panic]
fn test_binary_search_keys_invalid_cases() {
    struct KeyValue {
        key: i32,
    }

    impl Ord for KeyValue {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for KeyValue {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for KeyValue {}

    impl PartialEq for KeyValue {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    let keys = vec![
        KeyValue { key: 1 },
        KeyValue { key: 2 },
        KeyValue { key: 3 },
        KeyValue { key: 4 },
        KeyValue { key: 5 },
    ];

    // Since binary_search_keys is a method, calling on an invalid reference would panic
    let result = keys.binary_search_keys(&KeyValue { key: 6 });
    assert_eq!(result, Err(5));
}

