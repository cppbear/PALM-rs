// Answer 0

#[test]
fn test_as_slice_empty() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }

        fn as_slice(&self) -> &Slice<K, V> {
            Slice::from_slice(self.as_entries())
        }
    }

    let map: TestMap<i32, i32> = TestMap { entries: Vec::new() };
    let slice = map.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

#[test]
fn test_as_slice_non_empty() {
    struct TestMap<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> TestMap<K, V> {
        fn as_entries(&self) -> &[Bucket<K, V>] {
            &self.entries
        }

        fn as_slice(&self) -> &Slice<K, V> {
            Slice::from_slice(self.as_entries())
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "one" },
            Bucket { hash: HashValue::default(), key: 2, value: "two" },
        ],
    };
    let slice = map.as_slice();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
}

