// Answer 0

#[test]
fn test_reverse_empty() {
    struct TestEntries;

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>; 

        fn into_entries(self) -> Vec<Self::Entry> {
            Vec::new()
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }

        fn with_entries<F>(&mut self, _f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {}
    }

    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reverse();
    assert_eq!(index_map.len(), 0);
}

#[test]
fn test_reverse_single_entry() {
    struct SingleEntry;

    impl Entries for SingleEntry {
        type Entry = Bucket<usize, String>; 

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() }]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() }]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut [Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() }]
        }

        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    index_map.entries = SingleEntry.into_entries();

    index_map.reverse();
    assert_eq!(index_map.length(), 1);
    assert_eq!(index_map.entries[0].key, 1);
    assert_eq!(index_map.entries[0].value, "A");
}

#[test]
fn test_reverse_multiple_entries() {
    struct MultiEntry;

    impl Entries for MultiEntry {
        type Entry = Bucket<usize, String>; 

        fn into_entries(self) -> Vec<Self::Entry> {
            vec![
                Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() },
                Bucket { hash: HashValue::default(), key: 2, value: "B".to_string() },
                Bucket { hash: HashValue::default(), key: 3, value: "C".to_string() }
            ]
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &[
                Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() },
                Bucket { hash: HashValue::default(), key: 2, value: "B".to_string() },
                Bucket { hash: HashValue::default(), key: 3, value: "C".to_string() }
            ]
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut [
                Bucket { hash: HashValue::default(), key: 1, value: "A".to_string() },
                Bucket { hash: HashValue::default(), key: 2, value: "B".to_string() },
                Bucket { hash: HashValue::default(), key: 3, value: "C".to_string() }
            ]
        }

        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    index_map.entries = MultiEntry.into_entries();

    index_map.reverse();
    assert_eq!(index_map.length(), 3);
    assert_eq!(index_map.entries[0].key, 3);
    assert_eq!(index_map.entries[1].key, 2);
    assert_eq!(index_map.entries[2].key, 1);
}

