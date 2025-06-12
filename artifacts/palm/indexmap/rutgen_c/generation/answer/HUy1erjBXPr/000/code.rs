// Answer 0

#[test]
fn test_into_entries() {
    struct TestMapCore<K, V> {
        indices: Indices,
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> crate::Entries for TestMapCore<K, V> {
        type Entry = Bucket<K, V>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, _: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "value1" },
        Bucket { hash: HashValue(2), key: 2, value: "value2" },
    ];

    let test_map = TestMapCore {
        indices: Indices::new(),
        entries,
    };

    let result = test_map.into_entries();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].key, 1);
    assert_eq!(result[0].value, "value1");
    assert_eq!(result[1].key, 2);
    assert_eq!(result[1].value, "value2");
}

