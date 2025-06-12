// Answer 0

#[test]
fn test_into_ref_mut() {
    struct MockEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> super::Entries for MockEntries<K, V> {
        type Entry = (K, V);

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut mock_entries = MockEntries { data: vec![(1, "a"), (2, "b")] };
    let index = hashbrown::hash_table::OccupiedEntry::new(0);
    let raw_entry = super::RawOccupiedEntryMut {
        entries: &mut mock_entries,
        index,
        hash_builder: PhantomData,
    };

    let ref_mut = raw_entry.into_ref_mut();

    assert_eq!(ref_mut.entries.as_entries(), &[(1, "a"), (2, "b")]);
}

