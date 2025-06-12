// Answer 0

#[test]
fn test_into_entries() {
    struct MockIndexSet {
        map: MockIndexMap,
    }

    struct MockIndexMap {
        entries: Vec<Bucket<i32, ()>>,
    }

    impl MockIndexMap {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.entries
        }
    }

    impl Entries for MockIndexSet {
        type Entry = Bucket<i32, ()>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.map.into_entries()
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.map.entries
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.map.entries
        }

        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mock_set = MockIndexSet {
        map: MockIndexMap {
            entries: vec![
                Bucket { hash: 1, key: 1, value: () },
                Bucket { hash: 2, key: 2, value: () },
            ],
        },
    };

    let entries = mock_set.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 2);
}

