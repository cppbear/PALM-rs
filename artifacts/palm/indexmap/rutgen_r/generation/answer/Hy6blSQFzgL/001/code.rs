// Answer 0

#[test]
fn test_sorted_by_with_basic_comparison() {
    struct SimpleMap {
        entries: Vec<(i32, String)>,
    }

    impl SimpleMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            SimpleMap { entries }
        }

        fn into_entries(self) -> Vec<Entry<i32, String>> {
            self.entries.into_iter().map(|(k, v)| Entry { key: k, value: v }).collect()
        }
    }

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IntoIter<K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
    }

    impl<K, V> IntoIter<K, V> {
        fn new(entries: Vec<Entry<K, V>>) -> Self {
            IntoIter { entries, index: 0 }
        }
    }

    let map = SimpleMap::new(vec![(3, "three".to_string()), (1, "one".to_string()), (2, "two".to_string())]);

    let result: IntoIter<i32, String> = map.sorted_by(|k1, v1, k2, v2| {
        if k1 == k2 {
            Ordering::Equal
        } else if k1 < k2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let expected = vec![
        Entry { key: 1, value: "one".to_string() },
        Entry { key: 2, value: "two".to_string() },
        Entry { key: 3, value: "three".to_string() },
    ];

    assert_eq!(result.entries, expected);
}

#[test]
fn test_sorted_by_with_equal_keys() {
    struct SimpleMap {
        entries: Vec<(i32, String)>,
    }

    impl SimpleMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            SimpleMap { entries }
        }

        fn into_entries(self) -> Vec<Entry<i32, String>> {
            self.entries.into_iter().map(|(k, v)| Entry { key: k, value: v }).collect()
        }
    }

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IntoIter<K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
    }

    impl<K, V> IntoIter<K, V> {
        fn new(entries: Vec<Entry<K, V>>) -> Self {
            IntoIter { entries, index: 0 }
        }
    }

    let map = SimpleMap::new(vec![(2, "two1".to_string()), (1, "one".to_string()), (1, "one2".to_string())]);

    let result: IntoIter<i32, String> = map.sorted_by(|k1, v1, k2, v2| {
        if k1 == k2 {
            Ordering::Equal
        } else if k1 < k2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let expected = vec![
        Entry { key: 1, value: "one".to_string() },
        Entry { key: 1, value: "one2".to_string() },
        Entry { key: 2, value: "two1".to_string() },
    ];

    assert_eq!(result.entries, expected);
}

#[test]
fn test_sorted_by_with_reverse_order() {
    struct SimpleMap {
        entries: Vec<(i32, String)>,
    }

    impl SimpleMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            SimpleMap { entries }
        }

        fn into_entries(self) -> Vec<Entry<i32, String>> {
            self.entries.into_iter().map(|(k, v)| Entry { key: k, value: v }).collect()
        }
    }

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IntoIter<K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
    }

    impl<K, V> IntoIter<K, V> {
        fn new(entries: Vec<Entry<K, V>>) -> Self {
            IntoIter { entries, index: 0 }
        }
    }

    let map = SimpleMap::new(vec![(3, "three".to_string()), (2, "two".to_string()), (1, "one".to_string())]);

    let result: IntoIter<i32, String> = map.sorted_by(|k1, v1, k2, v2| {
        if k1 == k2 {
            Ordering::Equal
        } else if k1 < k2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let expected = vec![
        Entry { key: 3, value: "three".to_string() },
        Entry { key: 2, value: "two".to_string() },
        Entry { key: 1, value: "one".to_string() },
    ];

    assert_eq!(result.entries, expected);
}

