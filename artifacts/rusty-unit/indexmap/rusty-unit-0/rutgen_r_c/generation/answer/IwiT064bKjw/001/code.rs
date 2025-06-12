// Answer 0

#[test]
fn test_pop_empty_entries() {
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
    assert_eq!(index_map.pop(), None);
}

