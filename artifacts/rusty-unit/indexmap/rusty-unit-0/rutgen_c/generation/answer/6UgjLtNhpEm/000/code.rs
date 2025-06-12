// Answer 0

#[test]
fn test_first_empty() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
    }

    let map: TestMap<i32, i32> = TestMap { entries: vec![] };
    let result = map.first();
    assert_eq!(result, None);
}

#[test]
fn test_first_single_element() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
    }

    let map: TestMap<i32, i32> = TestMap {
        entries: vec![Bucket {
            hash: HashValue::default(),
            key: 1,
            value: 10,
        }],
    };
    let result = map.first();
    assert_eq!(result, Some((&1, &10)));
}

#[test]
fn test_first_multiple_elements() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> IndexMap<K, V, ()> {
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.core.entries
        }
    }

    let map: TestMap<i32, i32> = TestMap {
        entries: vec![
            Bucket {
                hash: HashValue::default(),
                key: 1,
                value: 10,
            },
            Bucket {
                hash: HashValue::default(),
                key: 2,
                value: 20,
            },
        ],
    };
    let result = map.first();
    assert_eq!(result, Some((&1, &10)));
}

