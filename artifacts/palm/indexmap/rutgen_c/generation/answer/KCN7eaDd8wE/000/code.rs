// Answer 0

#[test]
fn test_into_values() {
    struct TestBucket<K, V> {
        hash: u64,
        key: K,
        value: V,
    }

    struct TestIndexMap<K, V> {
        entries: Vec<TestBucket<K, V>>,
    }

    impl<K, V> TestIndexMap<K, V> {
        fn into_entries(self) -> Vec<TestBucket<K, V>> {
            self.entries
        }

        fn into_values(self) -> IntoValues<K, V> {
            IntoValues::new(self.into_entries())
        }
    }

    let map = TestIndexMap {
        entries: vec![
            TestBucket { hash: 1, key: "key1", value: "value1" },
            TestBucket { hash: 2, key: "key2", value: "value2" },
        ],
    };

    let values_iterator = map.into_values();
    let values: Vec<_> = values_iterator.iter.collect(); // Assume 'iter' gives access to iterated values.

    assert_eq!(values.len(), 2);
    assert_eq!(values[0].value, "value1");
    assert_eq!(values[1].value, "value2");
}

